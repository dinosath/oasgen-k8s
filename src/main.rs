use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::{PersistentVolume, Pod};

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions, conditions::is_pod_running},
    Client,
};
use tracing::info;

use axum::{
    extract::{rejection::FormRejection, Form, FromRequest},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use std::net::SocketAddr;
use validator::{Validate,ValidationError, ValidationErrors};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobBuilderRequest {
    pub openapis: Vec<Schema>,
    pub json_schemas: Vec<Schema>,
    pub template: Template,
}

impl Validate for JobBuilderRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let openapi_is_empty = self.openapis.len() == 0;
        if openapi_is_empty && self.json_schemas.len()==0 {
            let mut errors = ValidationErrors::new();
            let erroneous_field = if openapi_is_empty {"openapis"} else {"jsonSchemas"};
            errors.add(erroneous_field,ValidationError::new("must provide at least one openapi url or one jsonschema url"));
            return Err(errors);
        }
        Ok(())    
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub url: String,
    pub authentication: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub url: String,
    pub tag: Tag,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(rename = "type")]
    pub tag_type: Type,
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            name: "master".to_owned(),
            tag_type: Type::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    #[default]
    Branch,
    Tag,
    Revision,
}

async fn create_job(Json(payload): Json<JobBuilderRequest>) -> Result<(), AppError> {
    let client = Client::try_default().await?;

    let jobs: Api<Job> = Api::default_namespaced(client);
    let name = "oasgen-";

    let data = serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "generateName": name,
            "labels": {
                "app" : "oasgen"
            }
        },
        "spec": {
            "template": {
                "metadata": {
                    "name": "schema-tools"
                },
                "spec": {
                    "volumes":[
                        {
                            "name": "shared-data",
                            "emptyDir": {}
                        }
                    ],
                    "initContainers": [{
                        "name": "schema-tools",
                        "image": "ghcr.io/dinosath/schema-tools:master",
                        "args":["chain","-vvvv","-c",format!("registry add common https://github.com/dinosath/schema-tools-templates.git --branch master"),"-c","validate openapi https://petstore3.swagger.io/api/v3/openapi.json","-c","codegen openapi - --template common::rust/server-axum/ --target-dir /data/ -o namespace=client1 -o clientName=Client1"
                        ],
                        "volumeMounts":[
                            {
                                "name": "shared-data",
                                "mountPath": "/data"
                            }
                          ]
                    }],
                    "containers": [{
                        "name": "alpine",
                        "image": "alpine:latest",
                        "command": ["/bin/sh"],
                        "args":["ls","/data"],
                        "volumeMounts":[
                            {
                                "name": "shared-data",
                                "mountPath": "/data"
                            }
                          ]

                    }],
                    "restartPolicy": "Never",
                }
            }
        }
    }))?;
    info!("Creating job");

    jobs.create(&PostParams::default(), &data).await?;
    info!("Waiting for job to complete");
    let cond = await_condition(jobs.clone(), name, conditions::is_job_completed());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), cond).await?;

    info!("Cleaning up job record");
    jobs.delete(name, &DeleteParams::background()).await?;

    Ok(())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/schematools/build", post(create_job));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

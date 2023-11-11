#[macro_use]
extern crate enum_display_derive;
use std::default;
use std::fmt::Display;

use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::{PersistentVolume, Pod};

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions, conditions::is_pod_running},
    Client,
};
use tracing::info;

use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tracing_subscriber::fmt::format;

use std::net::SocketAddr;
use validator::{Validate, ValidationError, ValidationErrors};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorConfig {
    pub openapis: Vec<Schema>,
    pub json_schemas: Vec<Schema>,
    pub template: Template,
}

impl Validate for GeneratorConfig {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let openapi_is_empty = self.openapis.len() == 0;
        if openapi_is_empty && self.json_schemas.len() == 0 {
            let mut errors = ValidationErrors::new();
            let erroneous_field = if openapi_is_empty {
                "openapis"
            } else {
                "jsonSchemas"
            };
            errors.add(
                erroneous_field,
                ValidationError::new("must provide at least one openapi url or one jsonschema url"),
            );
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    #[validate(url)]
    pub url: String,
    pub tag: Option<Tag>,
    pub path: String,
    pub image: String,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            url: "https://github.com/dinosath/schema-tools-templates.git".to_string(),
            tag: Some(Tag::default()),
            path: "java/server-quarkus".to_string(),
            image: "ghcr.io/dinosath/schema-tools:master".to_string()
        }
    }
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Type {
    #[default]
    branch,
    tag,
    revision,
}

async fn create_job(
    extract::Json(generatorConfig): extract::Json<GeneratorConfig>,
) -> Result<(), AppError> {
    let client = Client::try_default().await?;

    let jobs: Api<Job> = Api::default_namespaced(client);
    let name = "oasgen-";

    let tag_substring = match generatorConfig.template.tag {
        Some(tag) => format!("--{} {}", tag.tag_type, tag.name),
        None => "".to_string(),
    };
    let add_registry_chain = format!(
        "registry add common {} {}",
        generatorConfig.template.url, tag_substring
    );

    
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
                        "image": generatorConfig.template.image,
                        "args":["chain","-vvvv","-c",add_registry_chain,"-c",format!("validate openapi {}",generatorConfig.openapis.get(0).unwrap().url),"-c",format!("codegen openapi - --template common::{} --target-dir /data/ -o package=com.test",generatorConfig.template.path)
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

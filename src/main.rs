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
    Json,
    routing::{get,post},
    Router,
};

use std::net::SocketAddr;
use validator::Validate;

use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobBuilderRequest {
    pub openapis: Vec<Openapi>,
    pub json_schemas: Vec<JsonSchema>,
    pub template_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Openapi {
    pub url: String,
    pub authentication: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    pub url: String,
    pub authentication: String,
}

async fn create_job(Json(payload): Json<JobBuilderRequest>,) -> Result<(), AppError> {
    let client = Client::try_default().await?;
    
    let jobs: Api<Job> = Api::default_namespaced(client);
    let name = "openapi-generator-";
    
    let data = serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "generateName": name,
        },
        "spec": {
            "template": {
                "metadata": {
                    "name": "empty-job-pod"
                },
                "spec": {
                    "containers": [{
                        "name": "empty",
                        "image": "alpine:latest"
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
    let app = Router::new()
        .route("/schematools/build", post(create_job));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

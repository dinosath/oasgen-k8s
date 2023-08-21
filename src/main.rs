use k8s_openapi::api::core::v1::{Pod, PersistentVolume};

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions::is_pod_running},
    Client,
};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;


    let pvs: Api<PersistentVolume> = Api::all(client);
    for pv in pvs.list(&ListParams::default().fields(("name=oasgen")).await? {
        println!("Found Pv: {}", pv.name_any());
    }

    // let pods: Api<Pod> = Api::namespaced(client, "apps");
    // let lp = ListParams::default().labels("app=blog"); // for this app only
    // for p in pods.list(&lp).await? {
    //     println!("Found Pod: {}", p.name_any());
    // }

    Ok(())

}

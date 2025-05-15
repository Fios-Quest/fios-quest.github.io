use dioxus::prelude::*;
use fios_quest::*;
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder().incremental(IncrementalRendererConfig::new()
            .static_dir(std::env::current_exe().unwrap().parent().unwrap()
            .join("public")).clear_cache(false)).enable_out_of_order_streaming()
        })
        .launch(App);
}

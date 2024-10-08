// src/main.rs

// dependencies
use shuttle_template_vite_vanilla_lib::{build, get_subscriber, init_subscriber};
use shuttle_template_vite_vanilla_lib::service::NasaImageryViewerService;
use shuttle_runtime::Error;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<NasaImageryViewerService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("shuttle-template-vite-vanilla".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // build the router
    let app_router  = build();

    // start the service
    Ok( NasaImageryViewerService { app_router })
}

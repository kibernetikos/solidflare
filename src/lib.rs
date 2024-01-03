use worker::*;

mod config;
mod dynamic_router;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            console_log!("Failed to load configuration: {:?}", e);
            return Response::error("Internal Server Error", 500);
        }
    };

    let router = dynamic_router::DynamicRouter::new(&config);

    // Use the parsed configuration
    console_log!("{:#?}", router);

    Response::ok("Hello, World!")
}

use crate::config::{Config, RouteConfig, ServiceConfig};
use js_sys::{Uint8Array, Array};
use serde_json::json;
use wasm_bindgen::JsValue;
use worker::*;

#[derive(Clone, Debug)]
pub struct DynamicRouter {
    routes: Vec<RouteConfig>,
    services: std::collections::HashMap<String, ServiceConfig>,
}

impl DynamicRouter {
    // Create a new DynamicRouter from the given configuration.
    pub fn new(config: &Config) -> Self {
        DynamicRouter {
            routes: config.routes.clone(),
            services: config.services.clone(),
        }
    }

    // Function to match a request and forward it to the appropriate service.
    pub async fn route_request(&self, req: Request, env: &Env, ctx: &Context) -> Result<Response> {
        let path = req.path();

        for route in &self.routes {
            if path.starts_with(&route.path) {
                return self.forward_request(req.clone_mut().unwrap(), route, env, ctx).await;
            }
        }

        Response::error("Not Found", 404)
    }

    // Forward the request to the appropriate service.
    async fn forward_request(
        &self,
        mut req: Request,
        route: &RouteConfig,
        env: &Env,
        ctx: &Context,
    ) -> Result<Response> {
        let service_config = self.services.get(&route.service);

        // Determine the base URL (either from the service_config or default to "https://localhost")
        let base_url = self.services.get(&route.service)
            .map(|config| config.url.as_str())
            .unwrap_or("https://localhost");

        let new_uri = format!("{}{}", base_url, req.path());
        let headers = req.clone().unwrap().headers().clone();
    
        let mut init = RequestInit::new();
        let body = req.bytes().await.unwrap().into_iter().map(JsValue::from).collect::<Array>();
        init.with_method(worker::Method::Post)
            .with_headers(headers)
            .with_body(Some(JsValue::from(body)));
    
        let request = Request::new_with_init(&new_uri, &init).unwrap();

        match service_config {
            Some(_) => {
                worker::Fetch::Request(request).send().await
            }
            None => {
                env.service(route.service.as_str())?
                    .fetch_request(request)
                    .await
            }
        }
    }
}

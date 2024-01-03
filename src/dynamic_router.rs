use crate::config::{Config, RouteConfig, ServiceConfig};
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
    pub async fn route_request(&self, req: Request, ctx: &RouteContext<()>) -> Result<Response> {
        let path = req.path();

        for route in &self.routes {
            if path.starts_with(&route.path) {
                return self.forward_request(req, route, ctx).await;
            }
        }

        Response::error("Not Found", 404)
    }

    // Forward the request to the appropriate service.
    async fn forward_request(&self, req: Request, route: &RouteConfig, ctx: &RouteContext<()>) -> Result<Response> {
        // Find the service configuration for the route.
        if let Some(service_config) = self.services.get(&route.service) {
            // Implement the logic to forward the request to the service specified in the route.
            // Use the service_config for details like URL, etc.
            Response::ok(format!("Forwarded to service: {} at URL: {}", route.service, service_config.url))
        } else {
            Response::error("Service not found", 404)
        }
    }
}

use crate::config::{Config, RouteConfig, ServiceConfig};
use js_sys::Array;
use wasm_bindgen::JsValue;
use worker::*;

/// Represents a dynamic router for forwarding requests to appropriate services.
#[derive(Clone, Debug)]
pub struct DynamicRouter {
    routes: Vec<RouteConfig>,
    services: std::collections::HashMap<String, ServiceConfig>,
}

impl DynamicRouter {
    /// Creates a new `DynamicRouter` from the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` object containing routing and service configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use dynamic_router::DynamicRouter;
    ///
    /// let config = config::load_config().unwrap();
    /// let router = DynamicRouter::new(&config);
    /// ```
    pub fn new(config: &Config) -> Self {
        DynamicRouter {
            routes: config.routes.clone(),
            services: config.services.clone(),
        }
    }

    /// Matches a request and forwards it to the appropriate service.
    ///
    /// # Arguments
    ///
    /// * `req` - The incoming `Request` object to be routed.
    /// * `env` - The `Env` object providing the runtime environment.
    /// * `ctx` - The `Context` object providing additional context information.
    ///
    /// # Returns
    ///
    /// A `Result<Response>` representing the response to the routed request.
    ///
    /// # Example
    ///
    /// ```
    /// use dynamic_router::DynamicRouter;
    /// use worker::Request;
    ///
    /// let router = DynamicRouter::new(&config);
    /// let request = Request::new("/api/resource", worker::Method::Get);
    /// let response = router.route_request(request, &env, &ctx).await;
    /// ```
    pub async fn route_request(&self, req: Request, env: &Env, ctx: &Context) -> Result<Response> {
        let path = req.path();

        for route in &self.routes {
            if path.starts_with(&route.path) {
                return self.forward_request(req, route, env, ctx).await;
            }
        }

        Response::error("Not Found", 404)
    }

    /// Forwards the request to the appropriate service.
    ///
    /// # Arguments
    ///
    /// * `req` - The incoming `Request` object to be forwarded.
    /// * `route` - The `RouteConfig` object specifying the routing configuration.
    /// * `env` - The `Env` object providing the runtime environment.
    /// * `ctx` - The `Context` object providing additional context information.
    ///
    /// # Returns
    ///
    /// A `Result<Response>` representing the response from the forwarded request.
    ///
    /// # Example
    ///
    /// ```
    /// use dynamic_router::DynamicRouter;
    /// use worker::Request;
    ///
    /// let router = DynamicRouter::new(&config);
    /// let route = RouteConfig { service: "example".to_string() };
    /// let request = Request::new("/api/resource", worker::Method::Get);
    /// let response = router.forward_request(request, &route, &env, &ctx).await;
    /// ```
    async fn forward_request(
        &self,
        req: Request,
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
        let body = req.clone().unwrap().bytes().await.unwrap().into_iter().map(JsValue::from).collect::<Array>();
        let method = req.method();
    
        let mut init = RequestInit::new();
        init.with_method(method)
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

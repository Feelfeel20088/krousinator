use crate::api::axum::auto_reg::AxumRouteMeta;
use axum::Router;
use inventory;

pub mod auto_reg;

pub fn register_axum_handlers() -> Router {
    let mut r = Router::new();
    for route in inventory::iter::<AxumRouteMeta> {
        println!("registering route: {}", route.path);
        r = (route.register_fn)(r);
    }
    r
}

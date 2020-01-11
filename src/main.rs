#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod lte_router;

use lte_router::Router;

fn main() {
    rocket::ignite().mount("/", routes![vpn_status, vpn_activate, vpn_deactivate]).launch();
}

/// Return VPN connection status
#[get("/vpn")]
fn vpn_status() -> String {
    let router = Router::new();
    router.vpn_status().unwrap()
}

#[get("/vpn/activate")]
fn vpn_activate() -> &'static str {
    let router = Router::new();
    router.vpn_activate();
    "OK"
}

#[get("/vpn/deactivate")]
fn vpn_deactivate() -> &'static str {
    let router = Router::new();
    router.vpn_deactivate();
    "OK"
}

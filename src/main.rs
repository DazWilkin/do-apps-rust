#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::config::{Config, ConfigError, Environment};
// use rocket::http::Status;
use rocket::request::{FromRequest, Request};

use rocket_prometheus::PrometheusMetrics;

use std::env;
use std::vec::Vec;

#[get("/")]
fn default() -> String {
    format!("Hello World!")
}

#[get("/cached")]
fn cached() -> String {
    format!("/cached")
}

#[get("/env")]
fn env() -> String {
    let envs: Vec<String> = std::env::vars()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect();
    format!("{:?}", envs)
}

#[get("/headers")]
fn headers(guard: Guard) -> String {
    format!("{:?}", guard.headers)
}

#[get("/healthz")]
fn healthz() -> String {
    format!("OK")
}

#[derive(Clone, Debug, PartialEq)]
struct Guard {
    headers: Vec<String>,
}

impl<'r, 'a> FromRequest<'r, 'a> for Guard {
    type Error = ();
    fn from_request(request: &'r Request<'a>) -> rocket::request::Outcome<Guard, ()> {
        let headers: Vec<String> = request
            .headers()
            .iter()
            .map(|header| format!("{}: {}", header.name(), header.value()))
            .collect();
        rocket::request::Outcome::Success(Guard { headers: headers })
    }
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() -> Result<(), ConfigError> {
    let key = "PORT";
    let port: u16 = match env::var(key) {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_) => 8080,
    };

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(port)
        .finalize()?;

    let prometheus = PrometheusMetrics::new();

    rocket::custom(config)
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/", routes![cached, default, env, headers, healthz, hello])
        .launch();

    Ok(())
}

use anyhow::{Context, Result};
use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;
use router::Router;
use serde::{Deserialize, Serialize};
use std::env;

fn main() {
    let mut router = Router::new();
    router.get("/", default_handler, "index");
    let port = get_port().unwrap();
    let addr = format!("0.0.0.0:{}", port);
    Iron::new(router).http(addr).unwrap();
}

fn get_port() -> Result<i32> {
    let port_str =
        env::var("HTTP_PORT").context("environment variable HTTP_PORT is not defined")?;
    let port: i32 = port_str
        .parse::<i32>()
        .with_context(|| format!("failed to parse {} as int", port_str))?;
    Ok(port)
}

#[derive(Serialize, Deserialize)]
struct ResponseSchema {
    version: String,
    headers: Vec<HTTPHeader>,
}

impl ResponseSchema {
    fn new(version: &str, headers: Vec<HTTPHeader>) -> ResponseSchema {
        ResponseSchema {
            version: version.to_string(),
            headers,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct HTTPHeader {
    key: String,
    value: String,
}

impl HTTPHeader {
    fn new(key: &str, value: &str) -> HTTPHeader {
        HTTPHeader {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

fn default_handler(req: &mut Request) -> IronResult<Response> {
    let mut headers: Vec<HTTPHeader> = vec![];
    for header in req.headers.iter() {
        let h = HTTPHeader::new(header.name(), &header.value_string());
        headers.push(h);
    }
    let rs = ResponseSchema::new(version::version!(), headers);
    let content = serde_json::to_string(&rs).unwrap();
    Ok(Response::with((status::Ok, content)))
}

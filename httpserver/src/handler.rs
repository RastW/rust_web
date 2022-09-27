use http::{httprequest::HttpRequest,httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    
    // 读取对应页面文件
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        fs::read_to_string(format!("{}/{}", public_path, file_name)).ok()
    }
}

pub struct WebServiceHandler;

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        todo!()
    }
}

pub struct PageNotFountHandler;

impl Handler for PageNotFountHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        todo!()
    }
}

pub struct StaticPageHandler;

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        todo!()
    }
}

use http::httprequest::Resource;
use http::{httprequest::HttpRequest,httpresponse::HttpResponse};
use serde::__private::de::Content;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    
    // 读取对应页面文件
    fn load_file(file_name: &str) -> Option<String> {
        // 读取当当前包根目录
        let default_path = format!(
                "{}/public", env!("CARGO_MANIFEST_DIR")
        );
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        // 读取并返回
        fs::read_to_string(format!("{}/{}", public_path, file_name)).ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFountHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String
}


impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route = s.split("/").collect::<Vec<&str>>();
        match route[1] {
            // /api
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            // /api/health 健康检查
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => 
                match Self::load_file(path) {
                    Some(content) => {
                        let mut map: HashMap<&str, &str> = HashMap::new();
                        if path.ends_with(".css") {
                            map.insert("Content-Type", "text/css");
                        } else if path.ends_with(".js"){
                            map.insert("Content-Type", "text/javascript");
                        } else {
                            map.insert("Content-Type", "text/html");
                        }
                        HttpResponse::new("200", Some(map), Some(content))
                    }
                    None => HttpResponse::new("404", None, Self::load_file("404.html")),
                }
        }
    }
}


impl Handler for PageNotFountHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        todo!()
    }
}
use http::httprequest::Resource;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::__private::de::Content;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use rtools::map_tool;

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
    order_status: String,
}


// 静态页面处理器
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route = s.split("/").collect::<Vec<&str>>();
        match route[1] {
            // Path: /api
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            // Path: /api/health 健康检查
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path =>
                match Self::load_file(path) {
                    Some(content) => {
                        let mut map: HashMap<&str, &str> = HashMap::new();
                        if path.ends_with(".css") {
                            map.insert("Content-Type", "text/css");
                        } else if path.ends_with(".js") {
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


// 404处理器
impl Handler for PageNotFountHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

// 业务处理器
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(path) = &req.resource;
        let route = path.split("/").collect::<Vec<&str>>();
        match route[2] {
            // Path: /api/shipping/orders
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(
                    serde_json::to_string(&Self::load_json()).unwrap()
                );
                let mut headers: HashMap<&str, &str> =
                    map_tool::of("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html"))
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        // 读取当当前包根目录
        let default_path = format!(
            "{}/public", env!("CARGO_MANIFEST_DIR")
        );
        // todo: 这里的DATA_PATH 读取不到
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "order.json");

        // 解析json转为Vec<OrderStatus>
        let orders: Vec<OrderStatus> = serde_json::from_str(
            fs::read_to_string(full_path).unwrap().as_str()).unwrap();
        orders
    }
}
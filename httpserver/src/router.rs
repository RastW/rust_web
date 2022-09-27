use super::handler::{Handler, PageNotFountHandler, StaticPageHandler, WebServiceHandler};
use http::httprequest::Resource;
use http::{
    httprequest::{HttpRequest, Method},
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    // 接收到请求后转到route路由，选择对应的处理器handler处理
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Post => {}
            Method::Get => match &req.resource {
                Resource::Path(s) =>
                // s.split("/").collect::<Vec<&str>>();
                {
                    match s.split("/").collect::<Vec<&str>>()[0] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            resp.send_response(stream);
                        }
                    }
                }
            },
            Method::Uninitialized => {
                let resp: HttpResponse = PageNotFountHandler::handle(&req);
                resp.send_response(stream);
            }
        }
    }
}

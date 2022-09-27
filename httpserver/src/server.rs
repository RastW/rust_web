use super::router::Router;
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server {socket_addr}
    }

    // 运行对外服务
    pub fn run(self) {
        let collection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Runner on {}", self.socket_addr);

        // 处理请求流数据
        for stream in collection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();
            let req: String = String::from_utf8(read_buffer.to_vec()).unwrap();

            // 路由到对应handler处理
            // Router::route(req, &mut stream);
        }
    }
}
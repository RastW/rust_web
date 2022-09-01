use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Result, Write};

// Debug 实现debug
// PartialEq trait 实现比较
// Clone 让其本身可以实现克隆
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default<'a> for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "request success",
            headers: None,
            body: None,
        }
    }
}


impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> String {
        let res_clone = res.clone();
        format!("{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
                &res.version(),
                &res.status_code(),
                &res.status_text(),
                &res.headers(),
                &res.body().len(),
                &res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) =>  headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match status_code{
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Nof Found",
            "500" => "Internal Server Error",
            _ => "Not Fount",
        };
        response.body = body;
        response
    }

    fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        // let response_string = String::from(res);
        // let _ = write!(write_stream, "{}", response_string);;

        Ok(())
    }


    pub fn version(&self) -> &str {
        self.version
    }
    pub fn status_code(&self) -> &str {
        self.status_code
    }
    pub fn status_text(&self) -> &str {
        self.status_text
    }
    pub fn headers(&self) -> String {
        let map = self.headers.unwrap().clone();
        let mut headers_string = "".into();
        for (k, v) in map.iter() {
            headers_string = format!("{}{}:{}\r\n", headers_string, k, v);
        }
        headers_string
    }
    pub fn body(&self) -> &str {
        // 这里的& 可以看做(self.body)，是对其中body的引用
        // match表达式可以看做函数，当传入参数不是引用的时候也会尝试获取所有权
        // 所以直接写self.body相当于尝试通过&self 这个引用来挪动其中body的所有权，会报错
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
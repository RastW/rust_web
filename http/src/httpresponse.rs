use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Result, Write};
use std::str::pattern::Pattern;

// Debug 实现debug
// PartialEq trait 实现比较
// Clone 让其本身可以实现克隆
#[drive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a>{
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}


impl Default<'a> for HttpResponse<'a> {
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

impl<'a> HttpResponse<'a> {
    pub new (
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    )
}
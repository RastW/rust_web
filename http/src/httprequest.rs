use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Post,
    Get,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut parsed_method = Method::Get;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::from(""));

        let mut headers = HashMap::new();
        let mut parsed_msg_body = String::from("");

        for line in req.lines() {
            if line.contains("http") {
                // 请求行
                let (method, resource, version) = process_req_line(req);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                // head
                let (key, value) = process_req_header(req);
                headers.insert(key, value);
            } else if line.len() == 0 {
                // 空行

            } else {
                // 消息体
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: headers,
            msg_body: parsed_msg_body,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}

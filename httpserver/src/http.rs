#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,
}

pub fn parse_request(request: String) -> HTTPRequest {
    let split : Vec<String> = request.split("\r\n").map(|s| s.into()).collect();
    let mut split_iter = split.iter();
    // don't mind these various ways of crashing the server
    // TODO: might want to fix this
    let mut method_path_ver = split_iter.next().unwrap().split(" ");
    let method = method_path_ver.next().unwrap().to_string();
    let path = method_path_ver.next().unwrap().to_string();
    let version = method_path_ver.next().unwrap().to_string();
    let headers = split_iter.clone().take_while(|&h| h != "")
        .map(|h| {
            let mut parts = h.split(":");
            Header {
                name: parts.next().unwrap().to_owned(),
                value: parts
                    .map(|s|s.to_owned())
                    .collect::<Vec<String>>()
                    .join(":")
                    .trim_start()
                    .to_string()
            }
        }
        ).collect::<Vec<Header>>();
    let body = split_iter.last().unwrap().to_owned();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("We are probably in the future compared to unix epoch")
        .as_millis();
    println!("{} '{}' -> '{}' << '{}'", now, method, path, body);

    HTTPRequest {
        method,
        path,
        version,
        headers,
        body,
    }
}

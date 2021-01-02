use std::net::{TcpStream};
use std::io::{Write, Read, Result};
use std::ops::Add;

pub struct HttpClient {}

impl HttpClient {
    pub fn hostname_from_url(url: &str) -> (&str, u32) {
        const PROTOCOL_SEPARATOR_LEN: usize = "://".len();
        let start = url.find("://")
            .expect(format!("invalid url (unknown protocol): {}", url).as_str())
            .add(PROTOCOL_SEPARATOR_LEN);
        if start >= url.len() {
            panic!("invalid url (too short): {}", url);
        }
        let port = match &url[..start - PROTOCOL_SEPARATOR_LEN] {
            "https" => 443,
            "http" => 80,
            _ => panic!("invalid url (unknown protocol): {}", &url[..start - PROTOCOL_SEPARATOR_LEN]),
        };
        let end = url[start..].find("/");
        let hostname =
            if let Some(end) = end {
                &url[start..start + end]
            } else {
                &url[start..]
            };
        if let Some(port_start) = hostname.find(':') {
            let port = &hostname[port_start+1..]; // move past ':'
            let hostname = &hostname[..port_start];
            let port = port.parse::<u32>()
                .expect(format!("invalid url (failed to parse port): {}", port).as_str());
            (hostname, port)
        } else {
            (hostname, port)
        }
    }

    fn get(&mut self, url: String) -> Result<()> {
        let hostname = Self::hostname_from_url(url.as_str());
        use std::net::ToSocketAddrs;
        //let listener = TcpStream::connect()
        /*
        socket.write_all(b"\r\n")?;
        socket.flush()
         */
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn hostname_from_url_default_port_https() {
        let url = "http://www.x.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.1, 80);
    }

    #[test]
    pub fn hostname_from_url_default_port_http() {
        let url = "https://www.y.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.1, 443);
    }

    #[test]
    pub fn hostname_from_url_port_specified() {
        let url = "https://www.z.com:1503/msg";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.1, 1503);
    }

    #[test]
    pub fn hostname_from_url() {
        let url = "https://www.tibia.com/news/?subtopic=latestnews";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.0, "www.tibia.com");
    }

    #[test]
    pub fn hostname_from_url_no_path() {
        let url = "https://www.tibia.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.0, "www.tibia.com");
    }

    #[test]
    #[should_panic(expected = "invalid url (too short): https://")]
    pub fn hostname_too_short() {
        let url = "https://";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
    }

    #[test]
    pub fn hostname_from_url_http() {
        let url = "http://www.tibia.com/news/?subtopic=latestnews";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.0, "www.tibia.com");
    }

    #[test]
    pub fn hostname_from_url_no_path_http() {
        let url = "http://www.tibia.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
        assert_eq!(hostname.0, "www.tibia.com");
    }

    #[test]
    #[should_panic(expected = "invalid url (too short): http://")]
    pub fn hostname_too_short_http() {
        let url = "http://";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
    }

    #[test]
    #[should_panic(expected = "invalid url (unknown protocol): ftp")]
    pub fn unknown_protocol() {
        let url = "ftp://duckduckgo.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
    }

    #[test]
    #[should_panic(expected = "invalid url (unknown protocol): http.com")]
    pub fn unknown_protocol_no_protocol() {
        let url = "http.com";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
    }

    #[test]
    #[should_panic(expected = "invalid url (unknown protocol): ")]
    pub fn empty_url_string() {
        let url = "";
        let hostname = crate::client::HttpClient::hostname_from_url(url);
    }
}
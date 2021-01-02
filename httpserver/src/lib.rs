mod buffer;
mod http;
mod decoders;
mod url;

use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, Result};
use std::collections::HashMap;

fn send_response(socket: &mut TcpStream, response: Vec<u8>) -> Result<()> {
    let mut length = response.len().to_string().into_bytes().into_boxed_slice();
    socket.write_all(b"HTTP/1.1 200 OK\r\n")?;
    socket.write_all(b"Server: StealMyPC\r\n")?;
    socket.write_all(b"Content-Type: text/html;charset=utf-8\r\n")?;
    socket.write_all(b"Content-Length: ")?;
    socket.write_all(length.as_mut())?;
    socket.write_all(b"\r\n")?;
    socket.write_all(b"Connection: close\r\n")?;
    socket.write_all(b"\r\n")?;
    socket.write_all(response.into_boxed_slice().as_mut())?;
    socket.write_all(b"\r\n")?;
    socket.flush()
}

fn build_response(request: &http::HTTPRequest, files: &HashMap<String, String>) -> Result<Vec<u8>> {
    // TODO: allow routing to /x/{y} etc
    // TODO: and allow custom responses, not just html files, e.g. "/shutdown" shuts down the server or w/e
    let mut response: Vec<u8> = Vec::new();

    if let Some(file) = files.get(&request.path) {
        let file = std::fs::read_to_string(file).expect("Failed to read file.html");
        response.append(&mut file.into_bytes());
    } else {
        if request.path == "/kill" {
            response.extend_from_slice(b"Killed server with /kill command");
        } else if request.path == "/smile" {
            if let Some(content_type) = request.headers.iter().find(|h| h.name.to_lowercase() == "content-type") {
                if content_type.value.to_lowercase().contains("application/x-www-form-urlencoded") {
                    // decode
                    println!("Request body: '{}'", request.body);
                    let form_pairs = decoders::form_url_decode(request.body.as_str());
                    let decoded_joined =
                        form_pairs.into_iter()
                            .map(|it| format!("{} = {}", it.name, it.value))
                            .collect::<Vec<String>>()
                            .join("\n");
                    println!("Decoded:\n{}", decoded_joined);
                } else {
                    println!("Request body: '{}'", request.body);
                }
            }
        } else {
            // should probably return a 404 instead though
            response.extend_from_slice(b"<h1>hello world</h1><a href=\"file\">file</a><br><a href=\"/\">index</a>");
        }
    }

    Ok(response)
}

fn server_loop(listener: TcpListener, files: HashMap<String, String>) -> Result<()> {
    loop {
        let (mut socket, _) = listener.accept().expect("Failed to accept connection");
        let mut buffer: [u8; 4096 * 4] = [0; 4096 * 4];
        socket.read(&mut buffer)?;

        let recv_str = String::from_utf8(buffer.to_vec()
            .into_iter()
            .take_while(|&c| c != 0)
            .collect()
        ).unwrap_or(String::new());

        let request = http::parse_request(recv_str);
        //println!("{}", decoders::url_decode(&request.path));
        let response = build_response(&request, &files)?;
        send_response(&mut socket, response)?;


        if request.path == "/kill" { // kill after sending response
            println!("Received kill command, exiting");
            break;
        }
    }
    Ok(())
}

pub fn start(files: HashMap<String, String>) -> Result<()> {
    let addr = "127.0.0.1:80";
    println!("Starting webserver on address {}", addr);
    let listener = TcpListener::bind(addr).expect("Failed to bind address");
    server_loop(listener, files)
}

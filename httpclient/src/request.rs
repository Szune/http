use std::net::{TcpStream};
use std::io::{Write, Read, Result};

fn send(socket: &mut TcpStream, response: Vec<u8>) -> Result<()> {
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

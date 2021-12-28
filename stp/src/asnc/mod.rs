use std::io;
use tokio::net::TcpStream;
use crate::{RecvError, RecvResult, SendResult};

pub mod server;
pub mod client;


async fn read_exact_async(s: &TcpStream, buf: &mut [u8]) -> io::Result<()> {
    let mut red = 0;
    while red < buf.len() {
        s.readable().await?;
        match s.try_read(&mut buf[red..]) {
            Ok(0) => break,
            Ok(n) => {
                red += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn write_all_async(s: &TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut written = 0;

    while written < buf.len() {
        s.writable().await?;

        match s.try_write(&buf[written..]) {
            Ok(0) => break,
            Ok(n) => {
                written += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn send_string_async<D: AsRef<str>>(d: D, s: &TcpStream) -> SendResult {
    let bytes = d.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    write_all_async(s, &len_bytes).await?;
    write_all_async(s, bytes).await?;
    Ok(())
}

async fn recv_string_async(s: &TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    read_exact_async(s, &mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    read_exact_async(s, &mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
pub mod client;
pub mod server;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::{RecvError, SendError};

/// Отправляет четыре байта `data.len()`, а потом сами данные.
pub async fn send_string<Data: AsRef<str>>(data: Data, writer: &mut TcpStream) -> Result<(), SendError> {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

/// Читает четыре байта длины, а потом сами данные.
pub async fn recv_string(reader: &mut TcpStream) -> Result<String, RecvError> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

use crate::error::{RecvError, RecvResult, SendResult};
use std::io::{Read, Write};

pub mod client;
pub mod error;
pub mod server;

fn send_string<Data: AsRef<str>, Writer: Write>(data: Data, mut writer: Writer) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

fn recv_string<Reader: Read>(mut reader: Reader) -> RecvResult {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

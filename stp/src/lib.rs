use crate::error::{RecvError, SendError};
use std::io::{Read, Write};

pub mod client;
pub mod error;
pub mod server;

/// Отправляет четыре байта `data.len()`, а потом сами данные.
fn send_string<Data: AsRef<str>, Writer: Write>(
    data: Data,
    mut writer: Writer,
) -> Result<(), SendError> {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

/// Читает четыре байта длины, а потом сами данные.
fn recv_string<Reader: Read>(mut reader: Reader) -> Result<String, RecvError> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

#[cfg(test)]
mod tests {
    use super::{recv_string, send_string};

    // Обратите внимание: generic реализация позволяет использовать в тестах
    // память, вместо реального сетевого обмена.

    #[test]
    fn test_send_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).unwrap();
        let result = recv_string(&buf[..]).unwrap();
        assert_eq!(data, result);
    }

    #[test]
    fn test_send() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).unwrap();

        let len = u32::from_be_bytes(buf[..4].try_into().unwrap());
        let string_data = String::from_utf8(buf[4..].to_vec()).unwrap();

        assert_eq!(data, string_data);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();
        buf.extend_from_slice(&5_u32.to_be_bytes());
        buf.extend_from_slice(data.as_bytes());

        let received = recv_string(&buf[..]).unwrap();
        assert_eq!(data, received);
    }
}

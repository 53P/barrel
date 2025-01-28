use std::io::{Cursor, Read};

use tokio::io::AsyncReadExt;

pub async fn decode_varint(cursor: &mut Cursor<Vec<u8>>) -> Result<i32, String> {
    let mut result = 0;
    let mut count = 0;

    loop {
        let byte = cursor.read_u8().await.unwrap();
        if count > 4 {
            return Err("Too many bytes; malformed".to_string());
        }
        let value = (byte & 0x7f) as i32;
        result |= value << count * 7;
        count += 1;

        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor as IoCursor;
    use std::io::Cursor;
    use std::io::Read;
    use std::vec::Vec;

    #[tokio::test]
    async fn test_decode_varint() {
        let mut cursor = Cursor::new(vec![0x05]);
        let result = decode_varint(&mut cursor).await.unwrap();
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn test_decode_varint_big() {
        let mut cursor = Cursor::new(vec![0x80, 0x02]);
        let result = decode_varint(&mut cursor).await.unwrap();
        assert_eq!(result, 256);
    }

    #[tokio::test]
    async fn test_decode_varint_malformed() {
        let mut cursor = Cursor::new(vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
        let result = decode_varint(&mut cursor).await;
        assert!(result.is_err());
    }
}

use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;

use crate::types::AppResult;

#[derive(Debug, Serialize)]
pub struct PaginatedQueryResult<D> {
    pub data: Vec<D>,
    pub pagination_info: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub has_next: bool,
}

pub fn encode_cursor(cursor: &str) -> String {
    let bytes = cursor.as_bytes();
    general_purpose::STANDARD.encode(bytes)
}

pub fn decode_cursor(cursor: &str) -> AppResult<String> {
    let decoded_bytes = general_purpose::STANDARD.decode(cursor)?;
    Ok(String::from_utf8_lossy(&decoded_bytes).to_string())
}

#[cfg(test)]
mod tests {
    use crate::types::Utc;

    use super::*;

    #[test]
    fn encode_decode_cursor() {
        let id = Utc::now().to_rfc3339();
        let encoded = encode_cursor(&id);
        let decoded = decode_cursor(&encoded).unwrap();
        assert_eq!(id, decoded);
    }
}

pub struct Transport;
impl Transport {
    pub fn content_length_header(body: &str) -> String {
        format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
    }
}

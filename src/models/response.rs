#[derive(Debug, Serialize, Deserialize)]
pub struct ResponesBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponesBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }        
    }
}
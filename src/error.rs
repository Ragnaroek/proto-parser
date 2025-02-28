#[derive(PartialEq, Debug)]
pub struct ProtoParseError {
    message: String,
}

pub fn err<T>(message: &str) -> Result<T, ProtoParseError> {
    return Err(ProtoParseError {
        message: message.to_string(),
    });
}

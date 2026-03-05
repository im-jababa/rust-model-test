/// 이메일 형식이 유효하지 않음
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidEmailFormatError;

impl std::fmt::Display for InvalidEmailFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid email format")
    }
}

impl std::error::Error for InvalidEmailFormatError {}

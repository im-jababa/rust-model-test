/// 인증 제공자 문자열 파싱 실패: 유효하지 않은 인증 제공자 문자열
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseProviderKindError;

impl std::fmt::Display for ParseProviderKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid provider kind")
    }
}

impl std::error::Error for ParseProviderKindError {}

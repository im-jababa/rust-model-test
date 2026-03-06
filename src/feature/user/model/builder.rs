use crate::feature::user::*;

#[derive(Debug, Clone)]
pub struct Builder {
    /// 인증 제공자
    pub provider: provider::Kind,

    /// 인증 제공자측 UID
    pub sub: provider::Subject,

    /// 이름
    pub name: String,

    /// 이메일 주소
    pub email: Option<Email>,
}

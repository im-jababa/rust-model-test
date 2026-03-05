use crate::feature::user::*;

/// 사용자 도메인
#[derive(Debug)]
pub struct User {
    /// 내부 아이디
    pk: PK,

    /// 외부 아이디
    id: ID,

    /// 인증 제공자
    provider: provider::Kind,

    /// 인증 제공자측 UID
    sub: provider::Subject,

    /// 이름
    name: String,

    /// 이메일 주소
    email: Option<Email>,
}

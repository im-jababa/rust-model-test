use crate::feature::user::*;

/// 사용자 Row 선택 결과
#[derive(Debug, Clone)]
pub struct SelectRow {
    /// 내부 아이디
    pub pk: PK,

    /// 외부 아이디
    pub id: ID,

    /// 인증 제공자
    pub provider: provider::Kind,

    /// 인증 제공자측 UID
    pub sub: provider::Subject,

    /// 이름
    pub name: String,

    /// 이메일 주소
    pub email: Option<Email>,

    /// 가입 시각
    pub created_at: chrono::NaiveDateTime,

    /// 마지막 정보 변경 시각
    pub updated_at: chrono::NaiveDateTime,
}

impl mysql_async::prelude::FromRow for SelectRow {
    fn from_row_opt(row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
    where
        Self: Sized,
    {
        Ok(Self {
            pk: row.get(0).expect("pk must exist"),
            id: row.get(1).expect("id must exist"),
            provider: row.get(2).expect("provider must exist"),
            sub: row.get(3).expect("sub must exist"),
            name: row.get(4).expect("name must exist"),
            email: row.get(5),
            created_at: row.get(6).expect("created_at must exist"),
            updated_at: row.get(7).expect("updated_at must exist"),
        })
    }
}

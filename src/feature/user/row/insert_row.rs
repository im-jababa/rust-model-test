use crate::feature::user::*;

/// 새로운 사용자 추가 쿼리 파라미터
#[derive(Debug, Clone)]
pub struct InsertRow {
    pub id: ID,
    pub provider: provider::Kind,
    pub sub: provider::Subject,
    pub name: String,
    pub email: Option<Email>,
}

// InsertRow -> Params
impl From<InsertRow> for mysql_async::Params {
    fn from(value: InsertRow) -> Self {
        use mysql_async::prelude::ToValue;
        mysql_async::Params::Positional(vec![
            value.id.to_value(),
            value.provider.to_value(),
            value.sub.to_value(),
            value.name.to_value(),
            value.email.map(|v| v.to_value()).to_value(),
        ])
    }
}

// Builder -> InsertRow
impl From<model::Builder> for InsertRow {
    fn from(value: model::Builder) -> Self {
        Self {
            id: ID::new(),
            provider: provider::Kind::Google,
            sub: value.sub,
            name: value.name,
            email: value.email,
        }
    }
}

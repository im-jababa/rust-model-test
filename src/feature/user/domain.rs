use crate::feature::user::*;
use mysql_async::prelude::{Queryable, ToValue};

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

    /// 가입 시각
    created_at: chrono::DateTime<chrono::Utc>,

    /// 마지막 정보 변경 시각
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// 새로운 사용자 생성
    ///
    /// ---
    /// # Arguments
    /// - `pool`: DB 연결 풀
    /// - `builder`: 사용자 생성 구조체
    ///
    /// ---
    /// # Returns
    /// - `Ok`: 생성 성공
    /// - `Err`: 실패
    ///
    pub async fn new(
        pool: &mysql_async::Pool,
        builder: model::Builder,
    ) -> Result<Self, mysql_async::Error> {
        let mut conn = pool.get_conn().await?;
        let stmt = include_str!("sql/new.sql");
        let params: row::InsertRow = builder.into();
        conn.exec_drop(stmt, params).await?;
        let pk: PK = conn.last_insert_id().expect("User PK must exist").into();
        drop(conn);

        let user = Self::find_pk(pool, pk).await?.expect("User must exist");
        Ok(user)
    }

    /// PK로 사용자 찾기
    ///
    /// ---
    /// # Arguments
    /// - `pool`: DB 연결 풀
    /// - `pk`: 사용자 테이블 PK
    ///
    /// ---
    /// # Returns
    /// - `Ok(Some)`: 성공, 사용자 찾음
    /// - `Ok(None)`: 성공, 사용자 없음
    /// - `Err`: 실패
    ///
    pub async fn find_pk(
        pool: &mysql_async::Pool,
        pk: PK,
    ) -> Result<Option<Self>, mysql_async::Error> {
        let mut conn = pool.get_conn().await?;
        let stmt = include_str!("sql/find_pk.sql");
        let params = (pk.to_value(),);
        let result: Option<row::SelectRow> = conn.exec_first(stmt, params).await?;
        Ok(result.map(|v| v.into()))
    }
}

impl From<row::SelectRow> for User {
    fn from(value: row::SelectRow) -> Self {
        Self {
            pk: value.pk,
            id: value.id,
            provider: value.provider,
            sub: value.sub,
            name: value.name,
            email: value.email,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

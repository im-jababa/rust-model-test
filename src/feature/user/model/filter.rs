use std::collections::HashSet;
use mysql_async::prelude::ToValue;
use crate::feature::user::*;

/// 사용자 목록 조회 필터
#[derive(Debug, Clone)]
pub struct Filter {
    /// 검색어 필터 (사용자 이름)
    pub keyword: Option<String>,

    /// 인증 제공자 필터
    pub provider: Option<HashSet<provider::Kind>>,

    /// 이메일 등록 여부 필터
    pub has_email: Option<bool>,
}

impl Filter {
    /// 쿼리 provider plcaeholder 생성
    /// ```ignore
    /// (? IS NULL OR `provider` IN ({})) <- 여기
    /// ```
    /// 
    /// ---
    /// # Returns
    /// - `String`: placeholder
    ///
    pub fn sql_provider_filter_placeholder(&self) -> String {
        match &self.provider {
            Some(provider) if !provider.is_empty() => vec!["?"; provider.len()].join(", "),
            _ => "NULL".to_string(),
        }
    }
}

impl From<Filter> for mysql_async::Params {
    fn from(value: Filter) -> Self {
        let Filter {
            keyword,
            provider,
            has_email,
        } = value;

        let mut params = Vec::new();

        let providers = provider
            .map(|provider| provider.into_iter().collect::<Vec<_>>())
            .unwrap_or_default();

        let provider_filter_exists = providers.first().is_some();
        let has_email_as_u8 = has_email.map(|v| if v { 1_u8 } else { 0_u8 });

        params.push(keyword.clone().into());
        params.push(keyword.clone().into());

        params.push(provider_filter_exists.into());
        for provider in providers {
            params.push(provider.to_value());
        }

        params.push(has_email_as_u8.into());
        params.push(has_email_as_u8.into());

        params.push(keyword.clone().into());
        params.push(keyword.clone().into());
        params.push(keyword.clone().into());
        params.push(keyword.into());

        mysql_async::Params::Positional(params)
    }
}

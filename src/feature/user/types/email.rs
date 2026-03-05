/// 사용자 이메일 주소
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// 최소한의 이메일 형식을 검사합니다.
    ///
    /// ---
    /// # Returns
    /// - `true`: 통과
    /// - `false`: 유효하지 않음
    ///
    pub fn validate(&self) -> Result<(), crate::feature::user::error::InvalidEmailFormatError> {
        use crate::feature::user::error::InvalidEmailFormatError;
        let s = &self.0;

        if s.len() > 254 {
            return Err(InvalidEmailFormatError);
        }

        let mut parts = s.split('@');

        let local = parts.next();
        let domain = parts.next();

        if parts.next().is_some() {
            return Err(crate::feature::user::error::InvalidEmailFormatError);
        }

        let (Some(local), Some(domain)) = (local, domain) else {
            return Err(crate::feature::user::error::InvalidEmailFormatError);
        };

        if local.is_empty() || domain.is_empty() {
            return Err(crate::feature::user::error::InvalidEmailFormatError);
        }

        if !domain.contains('.') {
            return Err(crate::feature::user::error::InvalidEmailFormatError);
        }

        Ok(())
    }
}

// Email -> String
impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// &str -> Email?
impl std::str::FromStr for Email {
    type Err = crate::feature::user::error::InvalidEmailFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email = Self(String::from_str(s).unwrap());
        email.validate()?;
        Ok(email)
    }
}

// Value -> Email?
impl TryFrom<mysql_async::Value> for Email {
    type Error = mysql_async::FromValueError;

    fn try_from(value: mysql_async::Value) -> Result<Self, Self::Error> {
        match value {
            mysql_async::Value::Bytes(items) => {
                Ok(Self(String::from_utf8(items.to_vec()).map_err(|_| {
                    mysql_async::FromValueError(mysql_async::Value::Bytes(items))
                })?))
            }
            _ => Err(mysql_async::FromValueError(value)),
        }
    }
}

// Value -> Email?
impl mysql_async::prelude::FromValue for Email {
    type Intermediate = Email;
}

// Value -> Email
impl mysql_async::prelude::ToValue for Email {
    fn to_value(&self) -> mysql_async::Value {
        mysql_async::Value::Bytes(self.0.as_bytes().to_vec())
    }
}

// Serialize
impl serde::Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = self.to_string();
        serializer.serialize_str(&string)
    }
}

// Deserialize
impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Self(string))
    }
}

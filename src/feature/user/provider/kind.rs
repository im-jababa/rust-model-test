/// 사용자 인증 제공자 유형 (OAuth)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Google,
}

// Kind -> String
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Google => write!(f, "google"),
        }
    }
}

// &str -> Kind?
impl std::str::FromStr for Kind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "google" => Ok(Self::Google),
            _ => Err(()),
        }
    }
}

// Value -> Kind?
impl TryFrom<mysql_async::Value> for Kind {
    type Error = mysql_async::FromValueError;

    fn try_from(value: mysql_async::Value) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        let string = match value {
            mysql_async::Value::Bytes(items) => {
                String::from_utf8(items).expect("String must encoded in utf8")
            }
            _ => return Err(mysql_async::FromValueError(value)),
        };
        Ok(Self::from_str(&string).map_err(|_| {
            mysql_async::FromValueError(mysql_async::Value::Bytes(string.as_bytes().to_vec()))
        })?)
    }
}

// Value -> Kind?
impl mysql_async::prelude::FromValue for Kind {
    type Intermediate = Kind;
}

// Kind -> Value
impl mysql_async::prelude::ToValue for Kind {
    fn to_value(&self) -> mysql_async::Value {
        mysql_async::Value::Bytes(self.to_string().as_bytes().to_vec())
    }
}

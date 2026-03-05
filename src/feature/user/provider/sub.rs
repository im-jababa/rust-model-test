/// 사용자 인증 제공자측 UID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Subject(String);

// Subject -> String
impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// &str -> Subject?
impl std::str::FromStr for Subject {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from_str(s)?))
    }
}

// Value -> Subject?
impl TryFrom<mysql_async::Value> for Subject {
    type Error = mysql_async::FromValueError;

    fn try_from(value: mysql_async::Value) -> Result<Self, Self::Error> {
        match value {
            mysql_async::Value::Bytes(items) => Ok(Self(
                String::from_utf8(items).expect("String must encoded in utf8"),
            )),
            _ => Err(mysql_async::FromValueError(value)),
        }
    }
}

// Value -> Subject?
impl mysql_async::prelude::FromValue for Subject {
    type Intermediate = Subject;
}

// Subject -> Value
impl mysql_async::prelude::ToValue for Subject {
    fn to_value(&self) -> mysql_async::Value {
        mysql_async::Value::Bytes(self.0.as_bytes().to_vec())
    }
}

// Serialize
impl serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let string = self.to_string();
        serializer.serialize_str(&string)
    }
}

// Deserialize
impl<'de> serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let string = String::deserialize(deserializer)?;
        Ok(Self(string))
    }
}

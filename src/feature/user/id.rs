/// 공개용 사용자 아이디
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ID(uuid::Uuid);

impl ID {
    /// 새로운 공개용 사용자 아이디 생성
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

// ID -> String
impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// &str -> ID?
impl std::str::FromStr for ID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

// Value -> ID?
impl TryFrom<mysql_async::Value> for ID {
    type Error = mysql_async::FromValueError;

    fn try_from(value: mysql_async::Value) -> Result<Self, Self::Error> {
        match value {
            mysql_async::Value::Bytes(items) => {
                let bytes: [u8; 16] = items
                    .try_into()
                    .map_err(|v| mysql_async::FromValueError(mysql_async::Value::Bytes(v)))?;
                Ok(Self(uuid::Uuid::from_bytes(bytes)))
            }
            _ => Err(mysql_async::FromValueError(value)),
        }
    }
}

// Value -> ID?
impl mysql_async::prelude::FromValue for ID {
    type Intermediate = ID;
}

// ID -> Value
impl mysql_async::prelude::ToValue for ID {
    fn to_value(&self) -> mysql_async::Value {
        mysql_async::Value::Bytes(self.0.as_bytes().to_vec())
    }
}

// Serialize
impl serde::Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = self.to_string();
        serializer.serialize_str(&string)
    }
}

// Deserialize
impl<'de> serde::Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;
        let string = String::deserialize(deserializer)?;
        Ok(Self::from_str(&string).map_err(|e| serde::de::Error::custom(e.to_string()))?)
    }
}

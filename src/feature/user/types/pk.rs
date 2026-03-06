/// 내부용 사용자 아이디
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PK(u64);

// u64 -> PK
impl From<u64> for PK {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// PK -> u64
impl From<PK> for u64 {
    fn from(value: PK) -> Self {
        value.0
    }
}

// PK -> String
impl std::fmt::Display for PK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// &str -> PK?
impl std::str::FromStr for PK {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

// Value -> PK?
impl TryFrom<mysql_async::Value> for PK {
    type Error = mysql_async::FromValueError;

    fn try_from(value: mysql_async::Value) -> Result<Self, Self::Error> {
        match value {
            mysql_async::Value::UInt(num) => Ok(Self(num)),
            _ => Err(mysql_async::FromValueError(value)),
        }
    }
}

// Value -> PK?
impl mysql_async::prelude::FromValue for PK {
    type Intermediate = PK;
}

// PK -> Value
impl mysql_async::prelude::ToValue for PK {
    fn to_value(&self) -> mysql_async::Value {
        mysql_async::Value::UInt(self.0)
    }
}

// Serialize
impl serde::Serialize for PK {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

// Deserialize
impl<'de> serde::Deserialize<'de> for PK {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let num = u64::deserialize(deserializer)?;
        Ok(Self(num))
    }
}

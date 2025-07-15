use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    pub fn as_null(&self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Null => Some(0),
            Self::Int(value) => Some(*value),
            Self::Float(value) => Some(*value as i64),
            Self::Bool(value) => Some(if *value { 1 } else { 0 }),
            Self::Str(value) => value.parse::<i64>().ok(),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Null => Some(0.0),
            Self::Int(value) => Some(*value as f64),
            Self::Float(value) => Some(*value),
            Self::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
            Self::Str(value) => value.parse::<f64>().ok(),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Null => Some(false),
            Self::Int(value) => Some(*value != 0),
            Self::Float(value) => Some(*value != 0.0),
            Self::Bool(value) => Some(*value),
            Self::Str(value) => match value.to_lowercase().as_str() {
                "false" | "0" | "off" | "no" | "" => Some(false),
                _ => Some(true),
            },
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Null => "null".to_string(),
            Self::Int(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Bool(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Self::Str(value) => value.clone(),
        };
        write!(f, "{string}")
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

impl FromStr for Value {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Str(s.to_string()))
    }
}

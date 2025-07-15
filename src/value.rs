use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    iter::{Product, Sum},
    ops::{Add, Mul},
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
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Int(value) => *value != 0,
            Self::Float(value) => *value != 0.0,
            Self::Bool(value) => *value,
            Self::Str(value) => !matches!(
                value.to_lowercase().as_str(),
                "false" | "0" | "off" | "no" | ""
            ),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    pub fn convert_to(&self, target_type: &Value) -> Option<Value> {
        match target_type {
            Value::Null => self.as_null().map(|_| Value::Null),
            Value::Int(_) => self.as_int().map(Value::Int),
            Value::Float(_) => self.as_float().map(Value::Float),
            Value::Bool(_) => Some(Value::Bool(self.as_bool())),
            Value::Str(_) => Some(Value::Str(self.to_string())),
        }
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

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Null, other) | (other, Self::Null) => other,

            (Self::Int(a), Self::Int(b)) => Self::Int(a + b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a + b),

            (Self::Int(i), Self::Float(f)) | (Self::Float(f), Self::Int(i)) => {
                Self::Float(i as f64 + f)
            }

            (Self::Bool(a), Self::Bool(b)) => Self::Bool(a || b),

            (b @ Self::Bool(_), other) | (other, b @ Self::Bool(_)) => {
                b.convert_to(&other).unwrap() + other
            }

            (Self::Str(a), Self::Str(b)) => Self::Str(a + &b),

            (Self::Str(s), Self::Int(i)) => Self::Str(s + &i.to_string()),
            (Self::Int(i), Self::Str(s)) => Self::Str(i.to_string() + &s),
            (Self::Str(s), Self::Float(f)) => Self::Str(s + &f.to_string()),
            (Self::Float(f), Self::Str(s)) => Self::Str(f.to_string() + &s),
        }
    }
}

impl Sum for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Null, |acc, value| acc + value)
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Null, _) | (_, Self::Null) => Self::Null,

            (Self::Int(a), Self::Int(b)) => Self::Int(a * b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a * b),

            (Self::Int(i), Self::Float(f)) | (Self::Float(f), Self::Int(i)) => {
                Self::Float(i as f64 * f)
            }

            (Self::Bool(a), Self::Bool(b)) => Self::Bool(a && b),

            (Self::Bool(b), other) | (other, Self::Bool(b)) => {
                if b {
                    other
                } else {
                    Self::Null
                }
            }

            (Self::Str(a), Self::Str(b)) => match (a.parse::<f64>(), b.parse::<f64>()) {
                (Ok(a), Ok(b)) => {
                    if a.fract() != 0.0 || b.fract() != 0.0 {
                        Self::Float(a * b)
                    } else {
                        Self::Int((a * b) as i64)
                    }
                }
                (Err(_), Ok(b)) => Self::Str(a.repeat(b as usize)),
                (Ok(a), Err(_)) => Self::Str(b.repeat(a as usize)),
                _ => Self::Null,
            },

            (Self::Str(s), Self::Int(i)) | (Self::Int(i), Self::Str(s)) => {
                Self::Str(s.repeat(i as usize))
            }
            (Self::Str(s), Self::Float(f)) | (Self::Float(f), Self::Str(s)) => {
                Self::Str(s.repeat(f as usize))
            }
        }
    }
}

impl Product for Value {
    fn product<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().unwrap_or(Self::Null);
        iter.fold(first, |acc, value| acc * value)
    }
}

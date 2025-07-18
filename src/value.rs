use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{Display, Formatter},
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Not, Rem, Sub},
    str::FromStr,
};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Value {
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

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Null => Some(0.0),
            Self::Int(value) => Some(*value as f64),
            Self::Float(value) => Some(*value),
            Self::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
            Self::Str(value) => value.parse::<f64>().ok(),
        }
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

    pub fn convert_to(&self, target_type: &Self) -> Option<Self> {
        match target_type {
            Self::Null => Some(Self::Null),
            Self::Int(_) => self.as_int().map(Self::Int),
            Self::Float(_) => self.as_float().map(Self::Float),
            Self::Bool(_) => Some(Self::Bool(self.as_bool())),
            Self::Str(_) => Some(Self::Str(self.to_string())),
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Int(_) => "int".to_string(),
            Self::Float(_) => "float".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Str(_) => "string".to_string(),
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

impl Default for Value {
    fn default() -> Self {
        Self::Null
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

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        // TODO: modify in-place instead of cloning
        *self = self.clone() + other;
    }
}

// an error raised when trying to perform operations on incompatible types
#[derive(Debug, Clone, Error, Diagnostic)]
#[error("Can't {operation} incompatible type{}: {}{}", if .b.is_some() { "s" } else { "" }, .a.type_name(), if let Some(right) = .b { format!(" and {}", right.type_name()) } else { String::new() })]
pub struct OperationIncompatibleTypesError {
    operation: String,
    pub a: Value,
    pub b: Option<Value>,
}

impl Neg for Value {
    type Output = Result<Self, OperationIncompatibleTypesError>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Null => Ok(Self::Null),
            Self::Int(value) => Ok(Self::Int(-value)),
            Self::Float(value) => Ok(Self::Float(-value)),
            Self::Bool(_) => Err(OperationIncompatibleTypesError {
                operation: "negate".to_string(),
                a: self,
                b: None,
            }),
            Self::Str(_) => Err(OperationIncompatibleTypesError {
                operation: "negate".to_string(),
                a: self,
                b: None,
            }),
        }
    }
}

impl Not for Value {
    type Output = Result<Self, OperationIncompatibleTypesError>;

    fn not(self) -> Self::Output {
        match self {
            Self::Null => Ok(Self::Null),
            Self::Int(value) => Ok(Self::Bool(value == 0)),
            Self::Float(value) => Ok(Self::Bool(value == 0.0)),
            Self::Bool(value) => Ok(Self::Bool(!value)),
            Self::Str(_) => Err(OperationIncompatibleTypesError {
                operation: "logically negate".to_string(),
                a: self,
                b: None,
            }),
        }
    }
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

impl Abs for Value {
    type Output = Result<Self, OperationIncompatibleTypesError>;
    fn abs(self) -> Self::Output {
        match self {
            Self::Null => Ok(Self::Null),
            Self::Int(value) => Ok(Self::Int(value.abs())),
            Self::Float(value) => Ok(Self::Float(value.abs())),
            Self::Bool(_) => Err(OperationIncompatibleTypesError {
                operation: "compute absolute value of".to_string(),
                a: self,
                b: None,
            }),
            Self::Str(_) => Err(OperationIncompatibleTypesError {
                operation: "compute absolute value of".to_string(),
                a: self,
                b: None,
            }),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, OperationIncompatibleTypesError>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (other, Self::Null) => Ok(other),

            (s @ Self::Str(_), other) => Err(OperationIncompatibleTypesError {
                operation: "subtract".to_string(),
                a: s,
                b: Some(other),
            }),

            (other, s @ Self::Str(_)) => Err(OperationIncompatibleTypesError {
                operation: "subtract".to_string(),
                a: other,
                b: Some(s),
            }),

            (Self::Null, other) => -other,

            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),

            (Self::Int(i), Self::Float(f)) => Ok(Self::Float(i as f64 - f)),
            (Self::Float(f), Self::Int(i)) => Ok(Self::Float(f - i as f64)),

            (a @ Self::Bool(_), b @ Self::Bool(_)) => {
                Ok(Self::Int(a.as_int().unwrap() - b.as_int().unwrap()))
            }

            (b @ Self::Bool(_), Self::Int(i)) => Ok(Self::Int(b.as_int().unwrap() - i)),
            (b @ Self::Bool(_), Self::Float(f)) => Ok(Self::Float(b.as_float().unwrap() - f)),
            (Self::Int(i), b @ Self::Bool(_)) => Ok(Self::Int(i - b.as_int().unwrap())),
            (Self::Float(f), b @ Self::Bool(_)) => Ok(Self::Float(f - b.as_float().unwrap())),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, OperationIncompatibleTypesError>;

    fn mul(self, other: Self) -> Self::Output {
        Ok(match (self, other) {
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

            (a @ Self::Str(_), b @ Self::Str(_)) => {
                return Err(OperationIncompatibleTypesError {
                    operation: "multiply".to_string(),
                    a,
                    b: Some(b),
                });
            }

            (Self::Str(s), Self::Int(i)) | (Self::Int(i), Self::Str(s)) => {
                if i.is_negative() {
                    Self::Str(
                        s.chars()
                            .rev()
                            .collect::<String>()
                            .repeat(i.unsigned_abs() as usize),
                    )
                } else {
                    Self::Str(s.repeat(i as usize))
                }
            }
            (Self::Str(s), Self::Float(f)) | (Self::Float(f), Self::Str(s)) => {
                if f.is_sign_negative() {
                    Self::Str(s.chars().rev().collect::<String>().repeat(f.abs() as usize))
                } else {
                    Self::Str(s.repeat(f as usize))
                }
            }
        })
    }
}

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum DivisionError {
    #[error("Division by zero is not allowed")]
    DivisionByZero,

    #[error(transparent)]
    IncompatibleTypes(#[from] OperationIncompatibleTypesError),
}

impl Div for Value {
    type Output = Result<Self, DivisionError>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Null, Self::Null) => Ok(Self::Null),
            (Self::Null, other) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "divide".to_string(),
                    a: Self::Null,
                    b: Some(other),
                },
            )),
            (other, Self::Null) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "divide".to_string(),
                    a: other,
                    b: Some(Self::Null),
                },
            )),

            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Int(a / b))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(a / b))
                }
            }

            (Self::Int(i), Self::Float(f)) => {
                if f == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(i as f64 / f))
                }
            }
            (Self::Float(f), Self::Int(i)) => {
                if i == 0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(f / i as f64))
                }
            }

            (Self::Bool(a), Self::Bool(b)) => {
                if !b {
                    return Err(DivisionError::DivisionByZero);
                }

                Ok(Self::Int(
                    Self::Bool(a).as_int().unwrap() / Self::Bool(b).as_int().unwrap(),
                ))
            }

            (b @ Self::Bool(_), Self::Int(i)) => {
                if i == 0 {
                    return Err(DivisionError::DivisionByZero);
                }
                Ok(Self::Int(b.as_int().unwrap() / i))
            }
            (b @ Self::Bool(_), Self::Float(f)) => {
                if f == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(b.as_float().unwrap() / f))
                }
            }
            (Self::Int(i), Self::Bool(b)) => {
                if !b {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Int(i / Self::Bool(b).as_int().unwrap()))
                }
            }
            (Self::Float(f), Self::Bool(b)) => {
                if !b {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(f / Self::Bool(b).as_float().unwrap()))
                }
            }

            (s @ Self::Str(_), other) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "divide".to_string(),
                    a: s,
                    b: Some(other),
                },
            )),
            (other, s @ Self::Str(_)) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "divide".to_string(),
                    a: other,
                    b: Some(s),
                },
            )),
        }
    }
}

impl Rem for Value {
    type Output = Result<Self, DivisionError>;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Null, Self::Null) => Ok(Self::Null),
            (Self::Null, other) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "modulo".to_string(),
                    a: Self::Null,
                    b: Some(other),
                },
            )),
            (other, Self::Null) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "modulo".to_string(),
                    a: other,
                    b: Some(Self::Null),
                },
            )),

            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Int(a % b))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(a % b))
                }
            }

            (Self::Int(i), Self::Float(f)) => {
                if f == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(i as f64 % f))
                }
            }
            (Self::Float(f), Self::Int(i)) => {
                if i == 0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(f % i as f64))
                }
            }

            (Self::Bool(a), Self::Bool(b)) => {
                if !b {
                    return Err(DivisionError::DivisionByZero);
                }

                Ok(Self::Int(
                    Self::Bool(a).as_int().unwrap() % Self::Bool(b).as_int().unwrap(),
                ))
            }

            (b @ Self::Bool(_), Self::Int(i)) => {
                if i == 0 {
                    return Err(DivisionError::DivisionByZero);
                }
                Ok(Self::Int(b.as_int().unwrap() % i))
            }
            (b @ Self::Bool(_), Self::Float(f)) => {
                if f == 0.0 {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(b.as_float().unwrap() % f))
                }
            }
            (Self::Int(i), Self::Bool(b)) => {
                if !b {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Int(i % Self::Bool(b).as_int().unwrap()))
                }
            }
            (Self::Float(f), Self::Bool(b)) => {
                if !b {
                    Err(DivisionError::DivisionByZero)
                } else {
                    Ok(Self::Float(f % Self::Bool(b).as_float().unwrap()))
                }
            }

            (s @ Self::Str(_), other) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "modulo".to_string(),
                    a: s,
                    b: Some(other),
                },
            )),
            (other, s @ Self::Str(_)) => Err(DivisionError::IncompatibleTypes(
                OperationIncompatibleTypesError {
                    operation: "modulo".to_string(),
                    a: other,
                    b: Some(s),
                },
            )),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Null, _) | (_, Self::Null) => None,

            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),

            (Self::Int(i), Self::Float(f)) => (*i as f64).partial_cmp(f),
            (Self::Float(f), Self::Int(i)) => f.partial_cmp(&(*i as f64)),

            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),

            (b @ Self::Bool(_), Self::Int(i)) => b.as_int().unwrap().partial_cmp(i),
            (Self::Int(i), b @ Self::Bool(_)) => i.partial_cmp(&b.as_int().unwrap()),
            (b @ Self::Bool(_), Self::Float(f)) => b.as_float().unwrap().partial_cmp(f),
            (Self::Float(f), b @ Self::Bool(_)) => f.partial_cmp(&b.as_float().unwrap()),

            (Self::Str(a), Self::Str(b)) => a.partial_cmp(b),
            (_, Self::Str(_)) | (Self::Str(_), _) => None,
        }
    }
}

use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::vm::RuntimeError;
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    StrObject(String),
}
impl ObjectType {
    fn get_type_as_string(&self) -> &str {
        match self {
            ObjectType::StrObject(v) => v,
        }
    }
}

pub type GenericObject = ObjectType;

#[derive(Clone, Debug)]
pub enum GenericValueType {
    Bool(bool),
    Number(f64),
    Object(GenericObject),
    Nil,
}

impl GenericValueType {
    fn get_type_as_str(&self) -> String {
        match self {
            GenericValueType::Bool(_) => String::from("bool"),
            GenericValueType::Number(_) => String::from("number"),
            GenericValueType::Nil => String::from("nil"),
            GenericValueType::Object(obj) => {
                format!("object {}", obj.get_type_as_string())
            }
        }
    }
}

pub type GenericValue = GenericValueType;

impl GenericValue {
    pub fn from_bool(value: bool) -> GenericValue {
        GenericValue::Bool(value)
    }
    pub fn from_number(value: f64) -> GenericValue {
        GenericValue::Number(value)
    }
    pub fn from_none() -> GenericValue {
        GenericValue::Nil
    }
    pub fn from_string(value: String) -> GenericValue {
        GenericValue::Object(ObjectType::StrObject(value))
    }
}

impl Display for GenericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericValueType::Bool(v) => write!(f, "{}", v),
            GenericValueType::Number(v) => write!(f, "{}", v),
            GenericValueType::Nil => write!(f, "nil"),
            GenericValueType::Object(v) => match v {
                ObjectType::StrObject(v) => write!(f, "String<Object>: {}", v),
            },
        }
    }
}

impl GenericValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let GenericValueType::Bool(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        if let GenericValueType::Number(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let GenericValueType::Object(ObjectType::StrObject(v)) = self {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<GenericObject> {
        if let GenericValueType::Object(o) = self {
            Some(o.clone())
        } else {
            None
        }
    }
}

impl Add for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>; // Should be using Result, and define an error for compiler error to handler
    fn add(self, other: GenericValue) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValueType::Number(lhs + rhs))
            }
            (
                GenericValueType::Object(ObjectType::StrObject(s1)),
                GenericValueType::Object(ObjectType::StrObject(s2)),
            ) => Ok(GenericValue::from_string(s1.to_owned() + s2)),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Sub for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn sub(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValueType::Number(lhs - rhs))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Mul for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn mul(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValueType::Number(lhs * rhs))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Div for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn div(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                if *rhs == 0.0 {
                    Err(RuntimeError::InvalidOperation(
                        "could not divide by zero".to_string(),
                    ))
                } else {
                    Ok(GenericValueType::Number(lhs / rhs))
                }
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Neg for GenericValue {
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Result<Self, RuntimeError> {
        match self {
            GenericValue::Number(value) => Ok(GenericValue::Number(-value)),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                self.get_type_as_str(),
            )),
        }
    }
}

impl PartialEq for GenericValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GenericValueType::Number(v1), GenericValueType::Number(v2)) => v1 == v2,
            (GenericValueType::Bool(b1), GenericValueType::Bool(b2)) => b1 == b2,
            (
                GenericValueType::Object(ObjectType::StrObject(s1)),
                GenericValueType::Object(ObjectType::StrObject(s2)),
            ) => s1 == s2,
            _ => false,
        }
    }
}

impl Eq for GenericValue {}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueArray {
    pub values: Vec<GenericValue>,
    pub count: usize,
}

impl ValueArray {
    pub fn new(values: Vec<GenericValue>) -> ValueArray {
        ValueArray {
            count: values.len(),
            values,
        }
    }
    pub fn write_value_array(&mut self, value: GenericValue) {
        self.values.push(value);
        self.count += 1;
    }
}

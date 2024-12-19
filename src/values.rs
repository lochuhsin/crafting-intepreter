use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::vm::RuntimeError;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GenericValueType {
    Bool(bool),
    Number(f64),
    Nil,
}

impl GenericValueType {
    fn get_type_as_str(&self) -> &str {
        match self {
            GenericValueType::Bool(_) => "bool",
            GenericValueType::Number(_) => "number",
            GenericValueType::Nil => "nil",
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
}

impl Display for GenericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericValueType::Bool(v) => write!(f, "{}", v),
            GenericValueType::Number(v) => write!(f, "{}", v),
            GenericValueType::Nil => write!(f, "nil"),
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
}

impl Add for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>; // Should be using Result, and define an error for compiler error to handler
    fn add(self, other: GenericValue) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValueType::Number(lhs + rhs))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str().to_string(),
                other.get_type_as_str().to_string(),
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
                self.get_type_as_str().to_string(),
                other.get_type_as_str().to_string(),
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
                self.get_type_as_str().to_string(),
                other.get_type_as_str().to_string(),
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
                self.get_type_as_str().to_string(),
                other.get_type_as_str().to_string(),
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
                self.get_type_as_str().to_string(),
                self.get_type_as_str().to_string(),
            )),
        }
    }
}

use std::collections::HashMap;

use super::expressions::{EvaluateValue, EvaluationType};

pub struct Environment {
    values: HashMap<String, EvaluateValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::<String, EvaluateValue>::new(),
        }
    }
    pub fn define(&mut self, name: String, object: EvaluateValue) {
        self.values.insert(name, object);
    }

    pub fn get(&self, name: String) -> Option<&EvaluateValue> {
        self.values.get(&name)
    }
}

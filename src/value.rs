pub type Value = f64; // This represents the "Number" value in lolang

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueArray {
    pub values: Vec<Value>,
    pub count: usize,
}

impl ValueArray {
    pub fn new(values: Vec<Value>) -> ValueArray {
        ValueArray {
            count: values.len(),
            values,
        }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
        self.count += 1;
    }
}

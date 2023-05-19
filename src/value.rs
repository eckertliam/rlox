#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Number(n) => Value::Number(-n),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
        }
    }
}

#[derive(Debug)]
pub struct ValueArray {
    pub data: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_value(&mut self, value: Value) {
        self.data.push(value);
    }

    pub fn free(&mut self) {
        self.data.clear();
    }
}

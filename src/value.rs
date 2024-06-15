use crate::vm::ExeState;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    String(String),
    Function(fn(&mut ExeState) -> i32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}

use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use crate::{bytecode::ByteCode, parser::ParseProto, value::Value};

fn lib_print(state: &mut ExeState) -> i32 {
    println!("{}", state.stack[1]);
    0
}

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl ExeState {
    pub fn new() -> Self {
        let globals = HashMap::from([(String::from("print"), Value::Function(lib_print))]);

        Self {
            globals,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, proto: &ParseProto) {
        for code in proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::Call(func, _) => {
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function:  {func:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
            }
        }
    }

    fn set_stack(&mut self, dst: u8, v: Value) {
        let dst = dst as usize;
        match dst.cmp(&self.stack.len()) {
            Ordering::Equal => self.stack.push(v),
            Ordering::Less => self.stack[dst] = v,
            Ordering::Greater => panic!("fail in set stack"),
        }
    }
}

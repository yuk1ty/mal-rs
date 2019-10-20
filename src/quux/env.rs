use crate::quux::types::MalType;
use crate::quux::{raise_err, MalResult};
use std::collections::HashMap;

pub struct Env(HashMap<String, fn(MalType, MalType) -> MalResult>);

impl Env {
    pub fn init() -> Env {
        let mut repl_env: HashMap<String, fn(MalType, MalType) -> MalResult> = HashMap::new();
        repl_env.insert("+".to_string(), |a, b| match (a, b) {
            (MalType::Int(_a), MalType::Int(_b)) => Ok(MalType::Int(_a + _b)),
            _ => raise_err("Caught unexpected type while parsing `+`"),
        });
        repl_env.insert("-".to_string(), |a, b| match (a, b) {
            (MalType::Int(_a), MalType::Int(_b)) => Ok(MalType::Int(_a - _b)),
            _ => raise_err("Caught unexpected type while parsing `-`"),
        });
        repl_env.insert("*".to_string(), |a, b| match (a, b) {
            (MalType::Int(_a), MalType::Int(_b)) => Ok(MalType::Int(_a * _b)),
            _ => raise_err("Caught unexpected type while parsing `*`"),
        });
        repl_env.insert("/".to_string(), |a, b| match (a, b) {
            (MalType::Int(_a), MalType::Int(_b)) => {
                if _b == 0 {
                    Ok(MalType::Int(0))
                } else {
                    Ok(MalType::Int(_a / _b))
                }
            }
            _ => raise_err("Caught unexpected type while parsing `/`"),
        });

        Env(repl_env)
    }
}

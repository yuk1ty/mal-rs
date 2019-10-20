use crate::quux::types::MalType;
use crate::quux::MalErr::ErrString;

pub mod printer;
pub mod reader;
pub mod step0_repl;
pub mod step1_repl;
pub mod types;

#[derive(Debug)]
pub enum MalErr {
    ErrString(String),
}

pub type MalResult = Result<MalType, MalErr>;

pub fn raise_err(s: &str) -> MalResult {
    Err(ErrString(s.to_string()))
}

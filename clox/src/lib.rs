#![allow(unused)]

static DEBUG_TRACE_EXECUTION: bool = true;
static DEBUG_PRINT_CODE: bool = true;

pub mod chunk;
pub mod compile;
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

#![allow(unused)]

static DEBUG_TRACE_EXECUTION: bool = false;
static DEBUG_PRINT_CODE: bool = false;

pub mod chunk;
pub mod compile;
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

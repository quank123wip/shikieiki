use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
enum ProcedureRuntimeType {
    Sequence,
    Parallel,
}

#[derive(Clone)]
enum ProcedureResultType {
    None,
    STDBOOL,
    STDPERC,
    STDNUM
}

#[derive(Clone)]
enum ProcedureResultCalculateStrategy {
    None,
    MAX,
    MIN,
    SUM,
    AVER,
    MUL,
}

#[derive(Clone)]
enum ProcedureLogLevel {
    ALL,
    INFO,
    WARNING,
    ERROR,
}

#[derive(Clone)]
struct Procedure {
    RuntimeType: ProcedureRuntimeType,
    CalcTime: bool,
    Time: u32,
    TimeLimit: u32,
    Exec: Vec<String>,
    LogLevel: ProcedureLogLevel,
    Log: String,
    Results: Vec<i32>,
    Result: f64,
    ResultRatio: f64,
    ResultCalculateStrategy: ProcedureResultCalculateStrategy,
    Finished: bool,
    Prev: Vec<Rc<RefCell<Procedure>>>,
    Next: Vec<Rc<RefCell<Procedure>>>,
}

#[derive(Clone)]
struct Task {
    Info: String,
    Head: Rc<RefCell<Procedure>>,
    Time: u32,
    TimeLimit: u32,
    Result: f64,
    Cwd: std::path::PathBuf,
    Env: std::collections::HashMap<String, String>,
}

impl Task {
    fn Run() {}
}

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
    StdBool,
    StdPerc,
    StdNum,
}

#[derive(Clone)]
enum ProcedureResultCalculateStrategy {
    None,
    Max,
    Min,
    Sum,
    Avg,
    Mul,
}

#[derive(Clone)]
enum ProcedureLogLevel {
    All,
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
struct Procedure {
    runtime_type: ProcedureRuntimeType,
    calc_time: bool,
    time: u32,
    time_limit: u32,
    exec: Vec<String>,
    log_level: ProcedureLogLevel,
    log: String,
    results: Vec<i32>,
    result: f64,
    result_ratio: f64,
    result_calculate_strategy: ProcedureResultCalculateStrategy,
    finished: bool,
    prev: Vec<Rc<RefCell<Procedure>>>,
    next: Vec<Rc<RefCell<Procedure>>>,
}

#[derive(Clone)]
struct Task {
    info: String,
    head: Rc<RefCell<Procedure>>,
    time: u32,
    time_limit: u32,
    result: f64,
    cwd: std::path::PathBuf,
    env: std::collections::HashMap<String, String>,
}

impl Task {
    fn run() {}
}

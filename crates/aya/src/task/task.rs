enum ProcedureRuntimeType {
    Sequence,
    Parallel,
}

enum ProcedureCalculateStrategy {
    None,
    MAX,
    MIN,
    SUM,
    AVER,
    MUL,
}

enum ProcedureLogLevel {
    ALL,
    INFO,
    WARNING,
    ERROR,
}

#[derive(Clone)]
struct Procedure {
    RuntimeType: ProcedureRuntimeType,
    Exec: Vec<String>,
    LogLevel: ProcedureLogLevel,
    Log: String,
    Results: Vec<i32>,
    Result: f64,
    ResultRatio: f64,
    CalculateStrategy: ProcedureCalculateStrategy,
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
    Cwd: std::PathBuf,
    Env: std::collections::HashMap,
}

impl Task {
    fn Run() {}
}

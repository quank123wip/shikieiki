enum ProcedureRuntimeType {
    Sequence,
    Parallel
}

enum ProcedureCalculateStrategy {
    None,
    MAX,
    SUM,
    AVER,
    
}

#[derive(Clone)]
struct Procedure {
    RuntimeType: ProcedureRuntimeType,
    Time: bool,
    Exec: Vec<String>,
    CalculateStrategy: ProcedureCalculateStrategy,
    Finished: bool,
    Prev: Vec<Rc<RefCell<Procedure>>>,
    Next: Vec<Rc<RefCell<Procedure>>>,
}
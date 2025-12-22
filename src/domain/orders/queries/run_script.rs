use std::sync::Arc;
use dyon::{load_str, Module, Runtime, Call};

pub fn execute_script(script: String) -> String {
    let mut module = Module::new();

    if let Err(e) = load_str("main.dyon", Arc::new(script), &mut module) {
        return format!("load error: {}", e);
    }

    let module = Arc::new(module);
    let mut rt = Runtime::new();
    let call = Call::new("calc");

    //SINK
    match call.run_vec4::<[f64; 4]>(&mut rt, &module) {
        Ok(v) => format!("{v:?}"),
        Err(e) => e,
    }
}

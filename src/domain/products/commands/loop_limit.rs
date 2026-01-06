use std::time::{Duration, Instant};

pub fn run_loop(limit: u64) {
    let state = LoopState::new(limit);

    if !state.should_run() {
        return;
    }

    process_items(state);
}

struct LoopState {
    limit: u64,
    started: Instant,
}

impl LoopState {
    fn new(limit: u64) -> Self {
        Self {
            limit,
            started: Instant::now(),
        }
    }

    fn should_run(&self) -> bool {
        self.limit > 0
    }
}

fn process_items(state: LoopState) {
    let mut i: u64 = 0;
    let start = state.started;
    let limit = state.limit;

    //SINK
    while i < limit {
        handle_iteration(i);

        if start.elapsed() > Duration::from_secs(5) {
            break;
        }

        i += 1;
    }
}

fn handle_iteration(index: u64) {
    let _ = index.to_string();
}

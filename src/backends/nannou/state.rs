use crate::backends::{StatefulRenderer, nannou::NannouBackend};

impl StatefulRenderer for NannouBackend {
    // MARK: - Context state

    fn push_state(&mut self) {
        eprintln!("NannouBackend::push_state()");
    }

    fn pop_state(&mut self) {
        eprintln!("NannouBackend::pop_state()");
    }
}

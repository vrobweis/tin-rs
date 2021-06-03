use crate::backends::StatefulRenderer;

use super::NannouBackend;



impl StatefulRenderer for NannouBackend {
    // MARK: - Context state
    
    fn push_state(&mut self) {
        eprintln!("NannouBackend::push_state()");
        self.save_state = true;
    }
    
    fn pop_state(&mut self) {
        eprintln!("NannouBackend::pop_state()");
        self.save_state = false;
    }
}
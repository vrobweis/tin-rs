use crate::backends::StatefulRenderer;

use super::LuminanceBackend;



impl StatefulRenderer for LuminanceBackend {

    // MARK: - Context state
    
    fn push_state(&mut self) {
        eprintln!("LuminanceBackend::push_state()");
        self.save_state = true;
    }
    
    fn pop_state(&mut self) {
        eprintln!("LuminanceBackend::pop_state()");
        self.save_state = false;
    }
}
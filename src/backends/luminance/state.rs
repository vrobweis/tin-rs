use crate::backends::{luminance::LuminanceBackend, StatefulRenderer};

impl StatefulRenderer for LuminanceBackend {
    // MARK: - Context state

    fn push_state(&mut self) {
        //self.save_state = true;
    }

    fn pop_state(&mut self) {
        //self.save_state = false;
    }
}

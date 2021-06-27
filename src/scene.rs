/// The user should implement this for the TinScene
pub trait TScene {
    // TODO: Make the run method take in an implemented TScene type with a new() constructor instead of an instance of it, if possible, to reduce boilerplate.
    // Does this todo require higher kinded types?

    // fn new(&mut self) -> Self;

    // setup() is called one time, immediately before first call to update().
    fn setup() -> Self;

    // Drawing code show go in update, or methods called during update.
    fn update(&mut self);

    fn on_event(&mut self, event: crate::TinEvent);
}

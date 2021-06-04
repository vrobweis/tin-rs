

use crate::UInt;

use super::{
    context::get_tin_mut,
    view::TinView,
    event::TinEvent,
    scene::TScene
};

pub(crate) struct TinController<S, H> where S: TScene, H: Fn(TinEvent, &mut S, &mut TinView) {
    pub(crate) tin_view: TinView,
    pub(crate) scene: S,
    pub(crate) handler: H
}

impl<S, H> TinController<S, H> where S: TScene, H: Fn(TinEvent, &mut S, &mut TinView) {

    pub fn new(tin_view: TinView, mut scene: S, handler: H) -> Self {
        scene.setup();
        let controller = Self {
            tin_view, scene, handler
        };
        {
            let frame = controller.get_view().get_frame().clone();
            get_tin_mut().prepare(frame)
        }
        controller
    }

    pub fn get_view(&self) -> & TinView {
        &self.tin_view
    }

    pub fn get_view_mut(&mut self) -> & mut TinView {
        & mut self.tin_view
    }

    /// Create a TinView and assign to the controllers view field.
    pub fn make_view(&mut self, title: &'static str, width: UInt, height: UInt) {
        self.tin_view = TinView::new_from_dimensions(title, width, height);
    }
    

    /// TODO: Document this method.
    pub fn present(&mut self, mut scene: S) {
        scene.setup();
        self.scene = scene;
    }

    pub fn draw(scene: &mut S) {
        {
            //let mut tin = get_tin_mut();
            get_tin_mut().update_frame_count();
            get_tin_mut().prepare_for_update();
        }
        scene.update();
        
        // Performance debugging display should be rendered here
        {
            get_tin_mut().did_finish_update();
        }
        
    }

}
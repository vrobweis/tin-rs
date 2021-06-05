use crate::{CurrentBackend, event::TEventHandler, scene::TScene, view::TinView, backends::TBackend};

pub struct TinApp<S> where S: TScene {
    pub(crate) scene: S,
    pub(crate) handler: TEventHandler<S>,
    pub(crate) view: TinView
}

impl<S> TinApp<S> where S: TScene {
    pub fn app(scene: S) -> Self {
        TinApp{
            scene, handler: |_,_,_| {}, view: TinView::new("Default Title", crate::frame::TinFrame::default())
        }
    }

    pub fn event(mut self, event_handler: TEventHandler<S>) -> Self {
        self.handler = event_handler;
        self
    }

    pub fn view(mut self, view: TinView) -> Self {
        self.view = view;
        self
    }

    pub fn run(mut self) -> Result<(), ()> {
        self.scene.setup();
        {
            let frame = self.view.get_frame().clone();
            crate::context::get_tin_mut().prepare(frame)
        }
        CurrentBackend::run(self)
    }
}
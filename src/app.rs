use crate::{
    backends::TBackend,
    scene::TScene,
    view::{TView, TinView},
    CurrentBackend, UShort,
};

pub struct Tin<S>
where
    S: TScene + 'static,
{
    pub(crate) view: TinView,
    pub(crate) target_fps: UShort,
    phantom: std::marker::PhantomData<S>,
}

impl<S> Tin<S>
where
    S: TScene,
{
    pub fn app() -> Self {
        Tin {
            view: TinView::new("Default Title", crate::frame::TinFrame::default()),
            target_fps: 60,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn view(mut self, view: TinView) -> Self {
        self.view = view;
        self
    }

    pub fn run(self) -> Result<(), ()> {
        {
            let frame = self.view.get_frame().clone();
            crate::context::get_tin_mut().prepare(frame)
        }
        CurrentBackend::run::<S>(self)
    }

    /// TODO: Document this method.
    pub fn get_fps(&self) -> UShort {
        self.target_fps
    }

    /// TODO: Document this method.
    pub fn set_fps(&mut self, fps: UShort) {
        self.target_fps = fps;
    }
}

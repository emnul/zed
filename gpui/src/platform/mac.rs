mod atlas;
mod dispatcher;
mod event;
mod fonts;
mod geometry;
mod platform;
mod renderer;
mod sprite_cache;
mod window;

use cocoa::base::{BOOL, NO, YES};
pub use dispatcher::Dispatcher;
pub use fonts::FontSystem;
use platform::{MacMainThreadPlatform, MacPlatform};
use std::{rc::Rc, sync::Arc};
use window::Window;

pub(crate) fn main_thread_platform() -> Rc<dyn super::MainThreadPlatform> {
    Rc::new(MacMainThreadPlatform::default())
}

pub(crate) fn platform() -> Arc<dyn super::Platform> {
    Arc::new(MacPlatform::new())
}

trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self {
            YES
        } else {
            NO
        }
    }
}

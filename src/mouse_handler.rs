use core_graphics::display::CGPoint;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MouseError {
    #[error("Failed to initialize event source.")]
    CGEventSourceFailure,
}

pub(crate) fn move_silently() {}

#[inline(always)]
fn move_to(x: i32, y: i32) -> Result<(), Box<dyn error::Error>> {
    CGEvent::new_mouse_event(
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
            .or(Err(MouseError::CGEventSourceFailure))?,
        CGEventType::MouseMoved,
        CGPoint::new(x as _, y as _),
        CGMouseButton::Left,
    )
        .or(Err(MouseError::CGEventSourceFailure))?
        .post(CGEventTapLocation::HID);

    Ok(())
}

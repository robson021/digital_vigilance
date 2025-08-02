use core_graphics::display::CGPoint;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::error;
use thiserror::Error;

#[derive(Error, Debug)]
enum MouseError {
    #[error("Failed to initialize event source.")]
    CGEventSourceFailure,

    #[error("Failed to create event source.")]
    CGEventFailure,
}

#[inline(always)]
pub fn move_silently() {
    if let Err(e) = move_back_and_forth() {
        eprintln!("Failed to move the mouse back and forth: {e}");
    };
}

fn move_back_and_forth() -> Result<(), Box<dyn error::Error>> {
    let current_pos = get_position()?;
    let new_pos = CGPoint::new(current_pos.x + 0.1, current_pos.y + 0.1);

    move_to(new_pos)?;
    move_to(current_pos)?;

    Ok(())
}

#[inline]
fn get_position() -> Result<CGPoint, Box<dyn error::Error>> {
    Ok(CGEvent::new(get_event_source()?)
        .or(Err(MouseError::CGEventFailure))?
        .location())
}

#[inline]
fn move_to(point: CGPoint) -> Result<(), Box<dyn error::Error>> {
    CGEvent::new_mouse_event(
        get_event_source()?,
        CGEventType::MouseMoved,
        point,
        CGMouseButton::Left,
    )
    .or(Err(MouseError::CGEventFailure))?
    .post(CGEventTapLocation::HID);

    dbg!("Moved to {:?}", point);
    Ok(())
}

#[inline(always)]
fn get_event_source() -> Result<CGEventSource, MouseError> {
    CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .or(Err(MouseError::CGEventSourceFailure))
}

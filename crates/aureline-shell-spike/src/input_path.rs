//! Input seam.
//!
//! The spike does not own platform input; it declares the seam that a
//! later platform-input adapter will back. The seam exposes three
//! observable things: the event enum, the dispatcher function, and the
//! named entry point the benchmark lab wraps for latency traces.

use crate::hooks::Hook;

/// Key events the spike recognises. Intentionally small; IME composition
/// is modelled as a separate event because it rides a different hook.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// A printable text burst delivered by the platform text-input
    /// pipeline (post-IME-commit, post-dead-key).
    TextInput(String),
    /// A named key press without a text burst (e.g. arrow keys, Escape).
    KeyPress(NamedKey),
    /// IME composition update. The spike re-emits this verbatim so the
    /// renderer can route it to the overlay layer.
    ImeComposition(ImeComposition),
    /// Mouse button event in window coordinates.
    MouseButton {
        button: MouseButton,
        pressed: bool,
        x: u32,
        y: u32,
    },
    /// Mouse wheel / trackpad scroll in units of "lines" per axis.
    Scroll { dx: i32, dy: i32 },
    /// A monitor scale change, such as moving between displays.
    ScaleChange { new_scale: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedKey {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Escape,
    Enter,
    Tab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImeComposition {
    pub text: String,
    /// Caret offset inside the composition, in bytes.
    pub caret_byte_offset: usize,
}

/// The action an input event resolves to. The spike models just enough
/// vocabulary to exercise every hot-path hook the ADR lists.
#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    InsertText(String),
    MoveCaret(CaretMove),
    ChangeSelection(SelectionDelta),
    UpdateComposition(ImeComposition),
    Scroll { dx: i32, dy: i32 },
    ScaleChange { new_scale: f32 },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaretMove {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDelta {
    Cleared,
    ExtendedLeft,
    ExtendedRight,
}

/// Resolve an input event into an action. Pure; no I/O; no hook
/// emission. The dispatcher in [`dispatch`] is where hooks fire.
pub fn resolve(event: &InputEvent) -> InputAction {
    match event {
        InputEvent::TextInput(text) => InputAction::InsertText(text.clone()),
        InputEvent::KeyPress(named) => match named {
            NamedKey::ArrowLeft => InputAction::MoveCaret(CaretMove::Left),
            NamedKey::ArrowRight => InputAction::MoveCaret(CaretMove::Right),
            NamedKey::ArrowUp => InputAction::MoveCaret(CaretMove::Up),
            NamedKey::ArrowDown => InputAction::MoveCaret(CaretMove::Down),
            NamedKey::Enter | NamedKey::Escape | NamedKey::Tab => InputAction::None,
        },
        InputEvent::ImeComposition(composition) => {
            InputAction::UpdateComposition(composition.clone())
        }
        InputEvent::MouseButton { pressed, .. } => {
            if *pressed {
                InputAction::ChangeSelection(SelectionDelta::Cleared)
            } else {
                InputAction::None
            }
        }
        InputEvent::Scroll { dx, dy } => InputAction::Scroll { dx: *dx, dy: *dy },
        InputEvent::ScaleChange { new_scale } => InputAction::ScaleChange {
            new_scale: *new_scale,
        },
    }
}

/// The hook that a resolved action will cause the render path to fire.
/// This is the mapping the benchmark lab relies on: an event's latency
/// is measured against the hook it eventually triggers.
pub const fn action_hook(action: &InputAction) -> Option<Hook> {
    match action {
        InputAction::InsertText(_) => Some(Hook::ReflowLineRange),
        InputAction::MoveCaret(_) => Some(Hook::CaretMove),
        InputAction::ChangeSelection(_) => Some(Hook::SelectionChange),
        InputAction::UpdateComposition(_) => Some(Hook::ImeCompositionUpdate),
        InputAction::Scroll { .. } => Some(Hook::ScrollFrame),
        InputAction::ScaleChange { .. } => Some(Hook::MultiMonitorScaleChange),
        InputAction::None => None,
    }
}

/// Dispatch one event through the full input seam: resolve it, then
/// return the (action, hook) pair the render path will service.
pub fn dispatch(event: &InputEvent) -> (InputAction, Option<Hook>) {
    let action = resolve(event);
    let hook = action_hook(&action);
    (action, hook)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_input_reflows() {
        let (action, hook) = dispatch(&InputEvent::TextInput("hi".to_owned()));
        assert_eq!(action, InputAction::InsertText("hi".to_owned()));
        assert_eq!(hook, Some(Hook::ReflowLineRange));
    }

    #[test]
    fn arrow_left_is_a_caret_move() {
        let (_, hook) = dispatch(&InputEvent::KeyPress(NamedKey::ArrowLeft));
        assert_eq!(hook, Some(Hook::CaretMove));
    }

    #[test]
    fn scroll_is_a_scroll_frame() {
        let (_, hook) = dispatch(&InputEvent::Scroll { dx: 0, dy: -3 });
        assert_eq!(hook, Some(Hook::ScrollFrame));
    }

    #[test]
    fn scale_change_is_multi_monitor() {
        let (_, hook) = dispatch(&InputEvent::ScaleChange { new_scale: 2.0 });
        assert_eq!(hook, Some(Hook::MultiMonitorScaleChange));
    }

    #[test]
    fn ime_composition_is_its_own_hook() {
        let composition = ImeComposition {
            text: "漢".to_owned(),
            caret_byte_offset: 3,
        };
        let (_, hook) = dispatch(&InputEvent::ImeComposition(composition));
        assert_eq!(hook, Some(Hook::ImeCompositionUpdate));
    }

    #[test]
    fn no_op_keys_fire_no_hook() {
        let (action, hook) = dispatch(&InputEvent::KeyPress(NamedKey::Escape));
        assert_eq!(action, InputAction::None);
        assert_eq!(hook, None);
    }
}

//! Text-input normalization for IME and layout-safe typing.
//!
//! Shell surfaces consume platform input events, but they should not each
//! re-implement text entry semantics. This module provides a small, portable
//! state machine that turns IME preedit/commit notifications plus keyboard text
//! bursts into an action vocabulary suitable for editor-like surfaces.
//!
//! The contract is intentionally platform-agnostic: the desktop shell adapts its
//! windowing backend events into [`ImeEvent`] and [`TextKeyEvent`], then applies
//! the resulting [`TextInputAction`] to the focused surface.

use serde::{Deserialize, Serialize};

/// Modifier snapshot used to interpret key events for text entry.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextInputModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

impl TextInputModifiers {
    /// Returns true when <kbd>Ctrl</kbd> or the platform "logo" key is held.
    pub const fn ctrl_or_logo(self) -> bool {
        self.ctrl || self.logo
    }

    /// Returns true when the modifier state looks like an AltGr chord.
    ///
    /// Many platforms report AltGr as a combined <kbd>Ctrl</kbd>+<kbd>Alt</kbd>
    /// modifier. Text input should still be admitted when the platform has
    /// already produced a printable text burst.
    pub const fn likely_alt_gr(self) -> bool {
        self.ctrl && self.alt && !self.logo
    }
}

/// Subset of navigation/edit keys that matter for text surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextInputKeyCode {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,
    Backspace,
    Delete,
    Enter,
    Other,
}

/// Directional caret movement within a text surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaretMove {
    Left,
    Right,
    Up,
    Down,
    WordLeft,
    WordRight,
    LineStart,
    LineEnd,
    PageUp,
    PageDown,
}

/// IME preedit composition metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImeComposition {
    /// Preedit text.
    pub text: String,
    /// Caret offset inside `text`, in bytes.
    pub caret_byte_offset: usize,
}

/// IME notifications adapted from the windowing backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ImeEvent {
    Enabled,
    Disabled,
    Preedit {
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cursor: Option<(usize, usize)>,
    },
    Commit {
        text: String,
    },
}

/// Key press event adapted for text entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextKeyEvent {
    pub code: TextInputKeyCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub is_repeat: bool,
    #[serde(default)]
    pub is_dead_key: bool,
    #[serde(default)]
    pub modifiers: TextInputModifiers,
}

/// Action vocabulary for editor-like text surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextInputAction {
    InsertText {
        text: String,
    },
    DeleteBackward,
    DeleteForward,
    MoveCaret {
        movement: CaretMove,
        #[serde(default)]
        extend_selection: bool,
    },
    UpdateComposition {
        composition: ImeComposition,
    },
    ClearComposition,
}

/// Stateful normalizer for IME and key-derived text input.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextInputSession {
    ime_enabled: bool,
    composition: Option<ImeComposition>,
}

impl TextInputSession {
    /// Creates a new session with IME disabled and no active composition.
    pub const fn new() -> Self {
        Self {
            ime_enabled: false,
            composition: None,
        }
    }

    /// Returns true when IME is enabled for this session.
    pub const fn ime_enabled(&self) -> bool {
        self.ime_enabled
    }

    /// Returns the current IME composition, when present.
    pub fn composition(&self) -> Option<&ImeComposition> {
        self.composition.as_ref()
    }

    /// Returns true when there is non-empty preedit text.
    pub fn is_composing(&self) -> bool {
        self.composition
            .as_ref()
            .is_some_and(|composition| !composition.text.is_empty())
    }

    /// Clears composition state without emitting an action.
    pub fn force_clear_composition(&mut self) {
        self.composition = None;
    }

    /// Applies an IME event and returns the resulting action, when any.
    pub fn handle_ime_event(&mut self, event: ImeEvent) -> Option<TextInputAction> {
        match event {
            ImeEvent::Enabled => {
                self.ime_enabled = true;
                None
            }
            ImeEvent::Disabled => {
                self.ime_enabled = false;
                self.composition = None;
                Some(TextInputAction::ClearComposition)
            }
            ImeEvent::Preedit { text, cursor } => {
                if text.is_empty() {
                    self.composition = None;
                    return Some(TextInputAction::ClearComposition);
                }

                let caret_byte_offset = cursor.map(|(start, _)| start).unwrap_or(text.len());
                let composition = ImeComposition {
                    text,
                    caret_byte_offset,
                };
                self.composition = Some(composition.clone());
                Some(TextInputAction::UpdateComposition { composition })
            }
            ImeEvent::Commit { text } => {
                if text.is_empty() {
                    return None;
                }
                self.composition = None;
                Some(TextInputAction::InsertText { text })
            }
        }
    }

    /// Resolves a key event into a text-surface action.
    pub fn handle_key_event(&self, event: &TextKeyEvent) -> Option<TextInputAction> {
        if event.is_repeat {
            return None;
        }
        if event.is_dead_key {
            return None;
        }
        if self.is_composing() {
            // IME preedit phase owns key handling; consumers should apply IME
            // updates instead of mutating buffers directly.
            return None;
        }

        let extend_selection = event.modifiers.shift;

        match event.code {
            TextInputKeyCode::ArrowLeft => {
                let movement = if event.modifiers.logo {
                    CaretMove::LineStart
                } else if event.modifiers.ctrl || event.modifiers.alt {
                    CaretMove::WordLeft
                } else {
                    CaretMove::Left
                };

                Some(TextInputAction::MoveCaret {
                    movement,
                    extend_selection,
                })
            }
            TextInputKeyCode::ArrowRight => {
                let movement = if event.modifiers.logo {
                    CaretMove::LineEnd
                } else if event.modifiers.ctrl || event.modifiers.alt {
                    CaretMove::WordRight
                } else {
                    CaretMove::Right
                };

                Some(TextInputAction::MoveCaret {
                    movement,
                    extend_selection,
                })
            }
            TextInputKeyCode::ArrowUp => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::Up,
                    extend_selection,
                })
            }
            TextInputKeyCode::ArrowDown => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::Down,
                    extend_selection,
                })
            }
            TextInputKeyCode::Home => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::LineStart,
                    extend_selection,
                })
            }
            TextInputKeyCode::End => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::LineEnd,
                    extend_selection,
                })
            }
            TextInputKeyCode::PageUp => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::PageUp,
                    extend_selection,
                })
            }
            TextInputKeyCode::PageDown => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::MoveCaret {
                    movement: CaretMove::PageDown,
                    extend_selection,
                })
            }
            TextInputKeyCode::Backspace => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::DeleteBackward)
            }
            TextInputKeyCode::Delete => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::DeleteForward)
            }
            TextInputKeyCode::Enter => {
                if event.modifiers.ctrl_or_logo() || event.modifiers.alt {
                    return None;
                }
                Some(TextInputAction::InsertText {
                    text: "\n".to_string(),
                })
            }
            TextInputKeyCode::Other => {
                let text = event.text.as_deref()?;
                if text.is_empty() {
                    return None;
                }

                if event.modifiers.ctrl_or_logo() && !event.modifiers.likely_alt_gr() {
                    return None;
                }

                Some(TextInputAction::InsertText {
                    text: text.to_string(),
                })
            }
        }
    }
}

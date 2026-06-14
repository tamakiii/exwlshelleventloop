#![doc = include_str!("../README.md")]
pub mod actions;
pub mod build_pattern;
pub mod multi_window;
pub mod settings;

mod clipboard;
mod conversion;
mod error;
mod event;
mod proxy;
mod user_interface;

pub use iced_sessionlock_macros::to_session_message;

pub use error::Error;

/// Conversion used by the runtime to deliver session-lock lifecycle
/// messages to the application. Use the [`to_session_message`] macro to
/// implement it together with `TryInto<UnLockAction>`.
pub trait FromLockedInfo {
    fn get(info: LockedInfo) -> Self;
}

/// Payload of the message the runtime delivers once the compositor has
/// activated the lock (the `ext_session_lock_v1.locked` event): every output
/// is covered by a lock surface and no unlocked content is visible anymore.
/// This is the point where a locker can safely start authentication or
/// report readiness (for example for lock-before-suspend ordering).
#[derive(Debug, Clone, Copy)]
pub struct LockedInfo;

use iced_core::theme::Base as DefaultStyle;
use iced_core::theme::Style as Appearance;

/// Opt-out for clipboard initialization. Call this before starting the
/// runtime when your app has no text input and doesn't need paste/copy —
/// this skips spawning the always-on smithay-clipboard worker thread.
pub fn disable_clipboard() {
    clipboard::set_disabled();
}

pub type Result = std::result::Result<(), error::Error>;

pub use build_pattern::application;
pub use settings::Settings;

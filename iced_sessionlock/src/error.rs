use iced_futures::futures;
use sessionlockev::SessionLockEventError;

/// An error that occurred while running an application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed(Box<dyn std::error::Error + Send + Sync>),

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(iced_graphics::Error),

    #[error("Error during dispatch")]
    WaylandDispatchFailed(#[from] SessionLockEventError),

    /// The compositor denied or revoked the session lock
    /// (`ext_session_lock_v1.finished`), for example because another lock
    /// screen is already running.
    #[error("the compositor denied or revoked the session lock")]
    LockFinished,
}

impl From<iced_graphics::Error> for Error {
    fn from(error: iced_graphics::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}

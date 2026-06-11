use crate::reexport::{Anchor, Layer, WlRegion};
use iced_core::window::Id as IcedId;
use layershellev::reexport::xdg_positioner::{
    Anchor as PopupAnchor, ConstraintAdjustment as PopupConstraintAdjustment,
    Gravity as PopupGravity,
};
use layershellev::{
    NewInputPanelSettings, NewLayerShellSettings, NewXdgWindowSettings, PopupPlacement,
};

use futures::channel::oneshot;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct IcedXdgWindowSettings {
    /// The initial window size.
    pub size: Option<(u32, u32)>,
    /// Request client-side decorations instead of the default server-side mode.
    pub client_side_decorations: bool,
}

impl From<IcedXdgWindowSettings> for NewXdgWindowSettings {
    fn from(val: IcedXdgWindowSettings) -> Self {
        NewXdgWindowSettings {
            title: None,
            size: val.size,
            client_side_decorations: val.client_side_decorations,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IcedNewPopupSettings {
    pub size: (u32, u32),
    pub parent: Option<IcedId>,
    pub placement: PopupPlacement,
    pub anchor: PopupAnchor,
    pub gravity: PopupGravity,
    pub constraint_adjustment: PopupConstraintAdjustment,
}

impl IcedNewPopupSettings {
    /// Create popup settings for a popup of `size` anchored at `anchor_rect`
    /// (in the parent surface's local coordinates) on the `parent` surface.
    ///
    /// Defaults are applied: anchored at the bottom-left of the anchor
    /// rect, growing toward the top-right, with the compositor free to flip
    /// or slide the popup on either axis to keep it on-screen. Override any of
    /// them with the builder methods.
    pub fn new(parent: IcedId, size: (u32, u32), anchor_rect: (i32, i32, i32, i32)) -> Self {
        Self::build(Some(parent), size, PopupPlacement::Anchored(anchor_rect))
    }

    pub fn on_current_surface(size: (u32, u32), anchor_rect: (i32, i32, i32, i32)) -> Self {
        Self::build(None, size, PopupPlacement::Anchored(anchor_rect))
    }

    pub fn at_position(parent: IcedId, size: (u32, u32), position: (i32, i32)) -> Self {
        Self::build(Some(parent), size, PopupPlacement::Position(position))
    }

    pub fn at_position_on_current_surface(size: (u32, u32), position: (i32, i32)) -> Self {
        Self::build(None, size, PopupPlacement::Position(position))
    }

    fn build(parent: Option<IcedId>, size: (u32, u32), placement: PopupPlacement) -> Self {
        Self {
            size,
            parent,
            placement,
            anchor: PopupAnchor::BottomLeft,
            gravity: PopupGravity::TopRight,
            constraint_adjustment: PopupConstraintAdjustment::FlipX
                | PopupConstraintAdjustment::FlipY
                | PopupConstraintAdjustment::SlideX
                | PopupConstraintAdjustment::SlideY,
        }
    }

    /// Set which point of the anchor rect the popup is anchored to.
    pub fn anchor(mut self, anchor: PopupAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Set the direction the popup grows from the anchor point.
    pub fn gravity(mut self, gravity: PopupGravity) -> Self {
        self.gravity = gravity;
        self
    }

    /// Set how the compositor may adjust (flip/slide/resize) the popup for off-screen cases
    pub fn constraint_adjustment(
        mut self,
        constraint_adjustment: PopupConstraintAdjustment,
    ) -> Self {
        self.constraint_adjustment = constraint_adjustment;
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IcedNewMenuSettings {
    pub size: (u32, u32),
    pub gravity: PopupGravity,
}

type Callback = Arc<dyn Fn(&WlRegion) + Send + Sync>;

// Callback wrapper around dyn Fn(&WlRegion)
#[derive(Clone)]
pub struct ActionCallback(pub Callback);

impl std::fmt::Debug for ActionCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "callback function")
    }
}

impl ActionCallback {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&WlRegion) + Send + Sync + 'static,
    {
        ActionCallback(Arc::new(callback))
    }
}

/// One-shot reply channel of
/// [LayerShellCustomAction::ActivationTokenRequest]. Wrapped in an
/// `Arc<Mutex<..>>` so the action enum stays `Clone` (like
/// [ActionCallback]); only the first completion is delivered.
#[derive(Clone)]
pub struct ActivationTokenSender(Arc<Mutex<Option<oneshot::Sender<Option<String>>>>>);

impl std::fmt::Debug for ActivationTokenSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "activation token sender")
    }
}

impl ActivationTokenSender {
    /// Create the reply channel: hand the sender to
    /// [LayerShellCustomAction::ActivationTokenRequest] and await the
    /// receiver for the token. The receiver resolves to `None` when the
    /// compositor does not support `xdg_activation_v1`.
    pub fn channel() -> (Self, oneshot::Receiver<Option<String>>) {
        let (sender, receiver) = oneshot::channel();
        (Self(Arc::new(Mutex::new(Some(sender)))), receiver)
    }

    pub(crate) fn send(&self, token: Option<String>) {
        if let Some(sender) = self.0.lock().ok().and_then(|mut guard| guard.take()) {
            let _ = sender.send(token);
        }
    }
}

/// NOTE: DO NOT USE THIS ENUM DIERCTLY
/// use macro to_layer_message
#[derive(Debug, Clone)]
pub enum LayerShellCustomAction {
    AnchorChange(Anchor),
    LayerChange(Layer),
    AnchorSizeChange(Anchor, (u32, u32)),
    MarginChange((i32, i32, i32, i32)),
    SizeChange((u32, u32)),
    ExclusiveZoneChange(i32),
    KeyboardInteractivityChange(layershellev::reexport::KeyboardInteractivity),
    VirtualKeyboardPressed {
        key: u32,
    },
    // settings, info, single_tone
    NewLayerShell {
        settings: NewLayerShellSettings,
        id: IcedId,
    },
    SetInputRegion(ActionCallback),
    NewPopUp {
        settings: IcedNewPopupSettings,
        id: IcedId,
    },
    NewMenu {
        settings: IcedNewMenuSettings,
        id: IcedId,
    },
    NewBaseWindow {
        settings: IcedXdgWindowSettings,
        id: IcedId,
    },
    NewInputPanel {
        settings: NewInputPanelSettings,
        id: IcedId,
    },
    /// is same with WindowAction::Close(id)
    RemoveWindow,
    ForgetLastOutput,
    /// Request an xdg-activation token for the surface of this window, to
    /// pass to a newly spawned client through the `XDG_ACTIVATION_TOKEN`
    /// environment variable so the compositor hands it focus (the launcher /
    /// notification-daemon use case). The token is delivered through the
    /// [ActivationTokenSender] reply channel.
    ActivationTokenRequest {
        /// the application id of the client that will be activated, optional
        app_id: Option<String>,
        sender: ActivationTokenSender,
    },
}

/// Please do not use this struct directly
/// Use macro to_layer_message instead
#[derive(Debug, Clone)]
pub struct LayerShellCustomActionWithId(pub Option<IcedId>, pub LayerShellCustomAction);

impl LayerShellCustomActionWithId {
    pub fn new(id: Option<IcedId>, action: LayerShellCustomAction) -> Self {
        Self(id, action)
    }
}

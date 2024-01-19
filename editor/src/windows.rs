//! Windowing implementation for the editor.

/// Hosts the editor windows, processing events and rendering.
pub struct EditorWindowHost;

/// A window that can be hosted by the editor.
pub trait EditorWindow {}

/// Settings for an editor window.
pub struct EditorWindowSettings;

/// A panel that can be hosted within an editor window.
pub trait EditorPanel {}

/// Layout settings for an editor panel.
pub struct EditorPanelLayout;

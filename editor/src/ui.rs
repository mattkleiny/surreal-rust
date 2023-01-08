//! The user interface for the Surreal editor.

use serde::{Deserialize, Serialize};

use content::*;
use game::*;
use graphs::*;
use inspector::*;
use scene::*;
use undo::*;

mod content;
mod game;
mod graphs;
mod inspector;
mod scene;
mod undo;

/// The main window for the editor.
///
/// The editor window contains state for all panels in the editor UI, as well
/// as metadata about the current project and open scene.
pub struct EditorWindow {
  editor_context: EditorContext,
  _editor_layout: EditorWindowLayout,
  inspector: Inspector,
  _content_browser: ContentBrowser,
  _scene_view: SceneView,
  _preview_view: GameView,
  graph_editor: GraphEditor<u32>,
}

/// Top-level contextual information for the application editor state.
///
/// The context provides access to central persistence, settings, undo/redo, etc.
#[derive(Default)]
pub struct EditorContext {
  _undo_manager: UndoManager,
}

impl EditorWindow {
  /// Builds a new [`EditorWindow`].
  pub fn new() -> Self {
    Self {
      editor_context: EditorContext::default(),
      _editor_layout: EditorWindowLayout::default(),
      inspector: Inspector::default(),
      _content_browser: ContentBrowser::default(),
      _scene_view: SceneView::default(),
      _preview_view: GameView::default(),
      graph_editor: GraphEditor::from_graph(surreal::graphs::Graph::default()),
    }
  }
}

impl EditorWindow {
  /// Shows the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::SidePanel::new(egui::panel::Side::Right, "inspector")
      .frame(egui::Frame::none())
      .show(egui, |ui| {
        self.inspector.show(ui, &mut self.editor_context);
      });

    egui::CentralPanel::default().frame(egui::Frame::none()).show(egui, |ui| {
      self.graph_editor.show(ui, &mut self.editor_context);
    });
  }
}

/// The layout for the [`EditorWindow`] and it's panels.
///
/// Layouts are saved to disk per-project, and loaded when the editor is opened.
#[derive(Serialize, Deserialize)]
pub struct EditorWindowLayout {
  pub primary_inspector: PanelLayout,
  pub content_browser: PanelLayout,
  pub scene_view: PanelLayout,
  pub preview_view: PanelLayout,
  pub graph_editor: PanelLayout,
}

/// Where to position a single panel within an [`EditorWindowLayout`].
#[derive(Serialize, Deserialize)]
pub struct PanelLayout {
  position: PanelPosition,
  size: Option<surreal::maths::Vec2>,
}

/// Where to position a panel within an [`EditorWindow`].
#[derive(Serialize, Deserialize)]
pub enum PanelPosition {
  Floating,
  Center,
  LeftTopInner,
  LeftTopOuter,
  LeftBottomInner,
  LeftBottomOuter,
  RightTopInner,
  RightTopOuter,
  RightBottomInner,
  RightBottomOuter,
}

impl Default for EditorWindowLayout {
  fn default() -> Self {
    Self {
      primary_inspector: PanelLayout {
        position: PanelPosition::RightTopInner,
        size: None,
      },
      content_browser: PanelLayout {
        position: PanelPosition::RightTopOuter,
        size: None,
      },
      scene_view: PanelLayout {
        position: PanelPosition::Center,
        size: None,
      },
      preview_view: PanelLayout {
        position: PanelPosition::Center,
        size: None,
      },
      graph_editor: PanelLayout {
        position: PanelPosition::LeftTopInner,
        size: None,
      },
    }
  }
}

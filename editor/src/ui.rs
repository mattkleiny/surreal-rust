//! The user interface for the Surreal editor.

pub use panels::*;
use serde::{Deserialize, Serialize};
pub use undo::*;
pub use widgets::*;
pub use windows::*;

use crate::Project;

mod panels;
mod undo;
mod widgets;
mod windows;

/// The main [`EditorWindow`] for the editor.
///
/// The editor window contains state for all panels in the editor UI, as well
/// as metadata about the current project and open scene.
pub struct ProjectWindow {
  editor_context: EditorContext,
  _editor_layout: EditorWindowLayout,
  inspector: Inspector,
  content_browser: ContentBrowser,
  _scene_view: SceneView,
  _game_view: GameView,
  graph_editor: GraphEditor<u32>,
}

impl EditorWindow for ProjectWindow {
  fn create_window(&self) -> winit::window::WindowBuilder {
    winit::window::WindowBuilder::new()
      .with_title(&format!(
        "Surreal - {} ({})",
        &self.editor_context.project.details.name, &self.editor_context.project.details.path
      ))
      .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
      .with_resizable(true)
      .with_transparent(true)
      .with_visible(true)
      .with_window_icon(Some(load_editor_icon()))
  }

  fn on_ui(&mut self, ctx: &egui::Context) {
    egui::SidePanel::new(egui::panel::Side::Right, "inspector")
      .frame(egui::Frame::none())
      .show(ctx, |ui| {
        self.inspector.show(ui, &mut self.editor_context);
      });

    egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "content")
      .frame(egui::Frame::none())
      .show(ctx, |ui| {
        self.content_browser.show(ui, &mut self.editor_context);
      });

    egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
      // TODO: render tabs for each panel
      // self.scene_view.show(ui, &mut self.editor_context);
      // self.game_view.show(ui, &mut self.editor_context);
      self.graph_editor.show(ui, &mut self.editor_context);
    });
  }
}

/// Top-level contextual information for the application editor state.
///
/// The context provides access to central persistence, settings, undo/redo,
/// etc.
pub struct EditorContext {
  project: Project,
  _undo_manager: UndoManager,
}

/// Represents a panel that can be rendered in the [`ProjectWindow`].
pub trait EditorPanel {
  /// Renders the active panel in-context.
  fn show(&mut self, ui: &mut egui::Ui, context: &mut EditorContext);
}

impl ProjectWindow {
  /// Builds a new [`ProjectWindow`].
  pub fn new(project: Project) -> Self {
    Self {
      editor_context: EditorContext {
        project,
        _undo_manager: UndoManager::default(),
      },
      _editor_layout: EditorWindowLayout::default(),
      inspector: Inspector::default(),
      content_browser: ContentBrowser::default(),
      _scene_view: SceneView::default(),
      _game_view: GameView::default(),
      graph_editor: GraphEditor::from_graph(surreal::graphs::Graph::default()),
    }
  }
}

/// The layout for the [`ProjectWindow`] and it's panels.
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

/// Where to position a panel within an [`ProjectWindow`].
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

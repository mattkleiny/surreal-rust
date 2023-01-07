//! The user interface for the Surreal editor.

use serde::{Deserialize, Serialize};

use content::*;
use game::*;
use graphs::*;
use inspector::*;
use scene::*;

mod content;
mod game;
mod graphs;
mod inspector;
mod scene;

/// The main window for the editor.
pub struct EditorWindow {
  editor_layout: WindowLayout,
  primary_inspector: Inspector,
  content_browser: ContentBrowser,
  scene_view: SceneView,
  preview_view: GameView,
  graph_editor: GraphEditor<u32>,
}

impl EditorWindow {
  /// Builds a new [`EditorWindow`].
  pub fn new() -> Self {
    Self {
      editor_layout: WindowLayout::default(),
      primary_inspector: Inspector::default(),
      content_browser: ContentBrowser::default(),
      scene_view: SceneView::default(),
      preview_view: GameView::default(),
      graph_editor: GraphEditor::from_graph(surreal::graphs::Graph::default()),
    }
  }
}

impl EditorWindow {
  /// Renders the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::SidePanel::new(egui::panel::Side::Right, "inspector")
      .frame(egui::Frame::none())
      .show(egui, |ui| {
        self.primary_inspector.show(ui);
      });

    egui::CentralPanel::default().frame(egui::Frame::none()).show(egui, |ui| {
      self.graph_editor.show(ui, ui.available_rect_before_wrap());
    });
  }
}

/// The layout for the [`EditorWindow`] and it's components.
#[derive(Serialize, Deserialize)]
pub struct WindowLayout {
  pub primary_inspector: WidgetLayout,
  pub content_browser: WidgetLayout,
  pub scene_view: WidgetLayout,
  pub preview_view: WidgetLayout,
  pub graph_editor: WidgetLayout,
}

/// Where to position a single window within an [`WindowLayout`].
#[derive(Serialize, Deserialize)]
pub struct WidgetLayout {
  position: WidgetPosition,
  size: Option<surreal::maths::Vec2>,
}

/// Where to position a window within an [`EditorWindow`].
#[derive(Serialize, Deserialize)]
pub enum WidgetPosition {
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

impl Default for WindowLayout {
  fn default() -> Self {
    Self {
      primary_inspector: WidgetLayout {
        position: WidgetPosition::RightTopInner,
        size: None,
      },
      content_browser: WidgetLayout {
        position: WidgetPosition::RightTopOuter,
        size: None,
      },
      scene_view: WidgetLayout {
        position: WidgetPosition::Center,
        size: None,
      },
      preview_view: WidgetLayout {
        position: WidgetPosition::Center,
        size: None,
      },
      graph_editor: WidgetLayout {
        position: WidgetPosition::LeftTopInner,
        size: None,
      },
    }
  }
}

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
  primary_inspector: InspectorWidget,
  content_browser: ContentBrowserWidget,
  scene_view: SceneViewWidget,
  preview_view: GameViewWidget,
  graph_editor: GraphEditorWidget<u32>,
}

impl EditorWindow {
  /// Builds a new [`EditorWindow`].
  pub fn new() -> Self {
    Self {
      editor_layout: WindowLayout::default(),
      primary_inspector: InspectorWidget::default(),
      content_browser: ContentBrowserWidget::default(),
      scene_view: SceneViewWidget::default(),
      preview_view: GameViewWidget::default(),
      graph_editor: GraphEditorWidget::from_graph(surreal::graphs::Graph::default()),
    }
  }
}

impl EditorWindow {
  /// Renders the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::CentralPanel::default()
      .frame(egui::Frame::default().inner_margin(0.0))
      .show(egui, |ui| {
        // TODO: partition viewing space into a layout, remember the layout for future sessions
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

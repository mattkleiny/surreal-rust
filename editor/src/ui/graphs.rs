//! Graph editor UI for the Surreal editor.
//!
//! This module provides a [`GraphEditor`] widget for the [`egui`] UI framework, as well
//! as supporting tools for creating, editing, saving and loading graphs.
//!
//! The [`GraphEditor`] widget is a first-class citizen in the Surreal editor, and is
//! used to create and edit graphs for use in procedural generation, visual scripting,
//! and other tasks.

use egui::*;

use surreal::graphs::*;

use super::*;

const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 5.0;

/// An `egui` editor for [`Graph`]s.
pub struct GraphEditor<D> {
  _graph: UndoScope<Graph<D>>,
  zoom: f32,
}

/// Internal messages for the [`GraphEditor`].
// #[derive(Debug)]
// enum GraphEditorMessage {
//   SelectNode(NodeId),
//   DeleteNode(NodeId),
//   DisconnectPort { input: PortId, output: PortId },
//   MoveNode { node: NodeId, delta: Vec2 },
//   ConnectPortStarted { port: PortId },
//   ConnectPortEnded { input: PortId, output: PortId },
// }

impl<D> GraphEditor<D> {
  /// Creates a [`GraphEditor`] for the given [`Graph`].
  pub fn from_graph(graph: Graph<D>) -> Self {
    Self {
      _graph: UndoScope::new(graph),
      zoom: 1.0,
    }
  }

  /// Shows the [`GraphEditor`] in the given context.
  pub fn show(&mut self, ui: &mut Ui, _context: &mut EditorContext) {
    let rect = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(rect, Sense::hover());

    let background_color = if response.hovered() {
      Color32::from_rgb(0x1c, 0x1c, 0x1c)
    } else {
      Color32::from_rgb(0x1b, 0x1b, 0x1b)
    };

    // let cursor_pos = ui.ctx().input().pointer.hover_pos().unwrap_or(egui::Pos2::ZERO);
    // let cursor_in_editor = rect.contains(cursor_pos);
    // let cursor_in_finder = false;

    self.zoom = (self.zoom + ui.ctx().input().zoom_delta() - 1.).clamp(ZOOM_MIN, ZOOM_MAX);

    let painter = ui.painter();

    Self::paint_grid(painter, rect, self.zoom, background_color);

    // TODO: paint nodes
    // TODO: paint connections
    // TODO: paint finder (if open)
    // TODO: paint blackboard (if open)
    // TODO: paint minimap (if open)
    // TODO: paint inspector (if open)
  }

  fn paint_grid(painter: &Painter, rect: Rect, zoom: f32, background_color: Color32) {
    let spacing = zoom * 10.0;
    let thick_spacing = spacing * 10.0;

    let line_color = Color32::from_rgb(0x2b, 0x2b, 0x2b);
    let thick_color = Color32::from_rgb(0x3a, 0x3a, 0x3a);

    painter.rect_filled(rect, Rounding::none(), background_color);

    Self::paint_grid_lines(painter, rect, spacing, line_color);
    Self::paint_grid_lines(painter, rect, thick_spacing, thick_color);
  }

  fn paint_grid_lines(painter: &Painter, rect: Rect, spacing: f32, color: Color32) {
    let stroke = Stroke::new(1.0, color);

    let mut x = rect.left();
    let mut y = rect.top();

    while x < rect.right() {
      painter.vline(x, rect.top()..=rect.bottom(), stroke);
      x += spacing;
    }

    while y < rect.bottom() {
      painter.hline(rect.left()..=rect.right(), y, stroke);
      y += spacing;
    }
  }
}

use egui::*;

use surreal::graphs::*;

const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 5.0;

/// An `egui` editor for [`Graph`]s.
pub struct GraphEditor<D> {
  graph: Graph<D>,
  zoom: f32,
}

/// Internal messages for the [`GraphEditor`].
#[derive(Debug)]
enum GraphEditorMessage {
  SelectNode(NodeId),
  DeleteNode(NodeId),
  DisconnectPort { input: PortId, output: PortId },
  MoveNode { node: NodeId, delta: Vec2 },
  ConnectPortStarted { port: PortId },
  ConnectPortEnded { input: PortId, output: PortId },
}

impl<D> GraphEditor<D> {
  /// Creates a [`GraphEditor`] for the given [`Graph`].
  pub fn from_graph(graph: Graph<D>) -> Self {
    Self { graph, zoom: 1.0 }
  }

  /// Renders the graph editor in the given [`Context`] .
  pub fn show(&mut self, ui: &mut Ui, rect: Rect) {
    ui.allocate_rect(rect, Sense::hover());

    let cursor_pos = ui.ctx().input().pointer.hover_pos().unwrap_or(egui::Pos2::ZERO);
    let cursor_in_editor = rect.contains(cursor_pos);
    let cursor_in_finder = false;

    self.zoom = (self.zoom + ui.ctx().input().zoom_delta() - 1.).clamp(ZOOM_MIN, ZOOM_MAX);

    let painter = ui.painter();

    Self::paint_grid(painter, rect, self.zoom);
    // TODO: paint nodes
    // TODO: paint connections
  }

  fn paint_grid(painter: &Painter, rect: Rect, zoom: f32) {
    let spacing = zoom * 10.0;
    let thick_spacing = spacing * 10.0;

    let fill_color = Color32::from_rgb(0x20, 0x20, 0x20);
    let line_color = Color32::from_rgb(0x2b, 0x2b, 0x2b);
    let thick_color = Color32::from_rgb(0x3a, 0x3a, 0x3a);

    painter.rect_filled(rect, Rounding::none(), fill_color);

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

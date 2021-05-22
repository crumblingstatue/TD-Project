use macroquad::prelude::*;

#[derive(Default)]
struct Path {
    nodes: Vec<Vec2>,
}

impl Path {
    fn debug_draw(&self) {
        for w in self.nodes.windows(2) {
            let (n1, n2) = (w[0], w[1]);
            draw_line(n1.x, n1.y, n2.x, n2.y, 2.0, RED);
        }
    }
    fn project_next_at_mouse(&self) {
        let (mx, my) = mouse_position();
        match self.nodes.last() {
            Some(last) => {
                draw_line(last.x, last.y, mx, my, 2.0, YELLOW);
            }
            None => {
                draw_circle(mx, my, 8.0, YELLOW);
            }
        }
    }
}

#[macroquad::main("TD Project")]
async fn main() {
    let mut path = Path::default();
    let mut editing = true;
    loop {
        let (mx, my) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) && editing {
            path.nodes.push(Vec2::new(mx, my));
        }
        if is_key_pressed(KeyCode::C) {
            path.nodes.clear();
        }
        if is_key_pressed(KeyCode::E) {
            editing = !editing;
        }
        path.debug_draw();
        if editing {
            path.project_next_at_mouse();
        }
        next_frame().await
    }
}

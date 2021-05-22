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

struct Enemy {
    pos: Vec2,
    path_pos: usize,
}

impl Enemy {
    pub fn debug_draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 4.0, GREEN);
    }
    pub fn spawn_on_path(path: &Path) -> Self {
        Self {
            pos: path.nodes[0],
            path_pos: 0,
        }
    }
    pub fn advance(&mut self, path: &Path) {
        let next = match path.nodes.get(self.path_pos) {
            Some(next) => *next,
            None => return,
        };
        let speed = 2.4;
        let diff = next - self.pos;
        let distance_sq = diff.x.powf(2.0) + diff.y.powf(2.0);
        if distance_sq.sqrt() < speed {
            self.path_pos += 1;
            return;
        }
        let angle = diff.y.atan2(diff.x);
        let x_move = angle.cos() * speed;
        let y_move = angle.sin() * speed;
        self.pos.x += x_move;
        self.pos.y += y_move;
        draw_line(self.pos.x, self.pos.y, next.x, next.y, 2.0, BLUE);
    }
}

#[macroquad::main("TD Project")]
async fn main() {
    let mut path = Path::default();
    let mut editing = true;
    let mut enemies = Vec::new();
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
        if is_key_pressed(KeyCode::S) && !path.nodes.is_empty() {
            enemies.push(Enemy::spawn_on_path(&path));
        }
        path.debug_draw();
        for enemy in &mut enemies {
            enemy.advance(&path);
            enemy.debug_draw();
        }
        if editing {
            path.project_next_at_mouse();
        }
        next_frame().await
    }
}

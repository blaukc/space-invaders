use macroquad::prelude::*;

#[derive(Debug)]
pub struct Entity {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

pub trait EntityTrait {
    fn draw_entity(&self);
    fn move_entity(&mut self, delta_time: f32);
}

pub struct Player {
    pub entity: Entity,
}

impl EntityTrait for Player {
    fn draw_entity(&self) {
        let cx = self.entity.x;
        let cy = self.entity.y;
        let r = self.entity.size;
        draw_circle(cx, cy, r, YELLOW);
    }

    fn move_entity(&mut self, delta_time: f32) {
        if is_key_down(KeyCode::Right) {
            self.entity.x += self.entity.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            self.entity.x -= self.entity.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            self.entity.y += self.entity.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            self.entity.y -= self.entity.speed * delta_time;
        }

        self.entity.x = clamp(self.entity.x, 0.0, screen_width());
        self.entity.y = clamp(self.entity.y, 0.0, screen_height());
    }
}

impl Player {
    fn collides_with(&self, other: &Enemy) -> bool {
        let size = other.entity.size;
        let x_min = other.entity.x - size / 2.0;
        let y_min = other.entity.y - size / 2.0;
        let x_max = x_min + size;
        let y_max = y_min + size;
        let cx = self.entity.x;
        let cy = self.entity.y;
        let r = self.entity.size;

        let closest_x = x_min.max(cx.min(x_max));
        let closest_y = y_min.max(cy.min(y_max));

        let dx = cx - closest_x;
        let dy = cy - closest_y;

        return (dx * dx + dy * dy) <= (r * r)
    }

    pub fn has_collision(&self, others: &Vec<Enemy>) -> bool {
        others.iter().any(|enemy| self.collides_with(&enemy))
    }
}

pub struct Enemy {
    pub entity: Entity,
}

impl EntityTrait for Enemy {
    fn draw_entity(&self) {
        let square = &self.entity;
        draw_rectangle(
            square.x - square.size / 2.0,
            square.y - square.size / 2.0,
            square.size,
            square.size,
            GREEN,
        );
    }

    fn move_entity(&mut self, delta_time: f32) {
        self.entity.y += self.entity.speed * delta_time;
    }
}

impl Enemy {
    pub fn is_oob(&self) -> bool {
        self.entity.y < screen_height() + self.entity.size
    }
}


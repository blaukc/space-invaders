use macroquad::prelude::*;

use crate::models::{EntityTrait};

mod models;

fn add_squares(enemies: &mut Vec<models::Enemy>) {
    if rand::gen_range(0, 99) >= 97 {
        let size = rand::gen_range(16.0, 64.0);
        enemies.push(models::Enemy {
            entity: models::Entity {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            },
        });
    }
}


fn move_entities(enemies: &mut Vec<models::Enemy>, player: &mut models::Player, delta_time: f32) {
    for enemy in &mut *enemies {
        enemy.move_entity(delta_time);
    }

    enemies.retain(|square| square.is_oob());

    player.move_entity(delta_time);
}


fn draw_entities(enemies: &Vec<models::Enemy>, player: &models::Player) {
    for enemy in enemies {
        enemy.draw_entity();
    }
    player.draw_entity();
}

#[macroquad::main("Game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let mut gameover = false;

    let mut enemies: Vec<models::Enemy> = vec![];
    let mut player = models::Player {
        entity: models::Entity {
            size: 16.0,
            speed: 200.0,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        }
    }; 

    loop {
        clear_background(WHITE);

        let delta_time = get_frame_time();

        add_squares(&mut enemies);

        if !gameover { move_entities(&mut enemies, &mut player, delta_time) }

        if player.has_collision(&enemies) { gameover = true; }

        draw_entities(&enemies, &player);

        next_frame().await
    }
}
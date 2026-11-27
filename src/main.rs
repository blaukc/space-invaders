use macroquad::prelude::*;

use crate::models::{EntityTrait};

mod models;

fn add_squares(enemies: &mut Vec<models::Enemy>) {
    if rand::gen_range(0, 99) >= 95 {
        let size = rand::gen_range(16.0, 64.0);
        enemies.push(models::Enemy {
            entity: models::Entity {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                ..Default::default()
            },
        });
    }
}

fn add_bullets(bullets: &mut Vec<models::Bullet>, player: &models::Player) {
    if is_key_pressed(KeyCode::Space) {
        let size = 5.0;
        let (x, y) = player.bullet_pos();
        bullets.push(models::Bullet {
            entity: models::Entity {
                size,
                speed: -100.0,
                x,
                y,
                ..Default::default()
            },
        });
    }
}


fn move_entities(enemies: &mut Vec<models::Enemy>, player: &mut models::Player, bullets: &mut Vec<models::Bullet>, delta_time: f32) {
    for enemy in &mut *enemies {
        enemy.move_entity(delta_time);
    }

    enemies.retain(|square| square.is_oob());

    for bullet in bullets {
        bullet.move_entity(delta_time);
    }

    player.move_entity(delta_time);
}


fn draw_entities(enemies: &Vec<models::Enemy>, player: &models::Player, bullets: &Vec<models::Bullet>) {
    for enemy in enemies {
        enemy.draw_entity();
    }
    player.draw_entity();
    for bullet in bullets {
        bullet.draw_entity();
    }
}

fn bullet_collisions(enemies: &mut Vec<models::Enemy>, bullets: &mut Vec<models::Bullet>) {
    for bullet in &mut *bullets {
        bullet.update_collisions(enemies);
    }
    bullets.retain(|bullet| !bullet.entity.to_remove);
    enemies.retain(|enemy| !enemy.entity.to_remove);
}

fn restart_game(enemies: &mut Vec<models::Enemy>, player: &mut models::Player, bullets: &mut Vec<models::Bullet>) {
    enemies.clear();
    bullets.clear();
    player.reset();
}

fn display_gameover() {
    let text = "GAME OVER!";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        RED,
    );
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
            ..Default::default()
        }
    }; 
    let mut bullets: Vec<models::Bullet> = vec![];


    loop {
        clear_background(WHITE);
        let delta_time = get_frame_time();

        if gameover && is_key_pressed(KeyCode::Space) { 
            gameover = false;
            restart_game(&mut enemies, &mut player, &mut bullets);
        }
        if gameover { display_gameover(); }


        if !gameover { 
            add_squares(&mut enemies);
            add_bullets(&mut bullets, &player);
            move_entities(&mut enemies, &mut player, &mut bullets, delta_time) 
        }
        bullet_collisions(&mut enemies, &mut bullets);
        if player.has_collision(&enemies) { gameover = true; }
        draw_entities(&enemies, &player, &bullets);

        next_frame().await
    }
}
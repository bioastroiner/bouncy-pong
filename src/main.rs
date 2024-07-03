use macroquad::{
    prelude::*,
    window::{self, Conf as WindowCFG},
};
use miniquad::{Context, EventHandler};
fn window_conf() -> WindowCFG {
    WindowCFG {
        window_title: "Bouncy Pong".to_owned(),
        window_resizable: true,
        ..Default::default()
    }
}

struct Transform {
    pos: Vec2,
    size: Vec2,
}

struct Object {
    transform: Transform,
    velocity: Vec2,
}

impl Object {
    fn rect(&self) -> Rect {
        Rect::new(
            self.transform.pos.x,
            self.transform.pos.y,
            self.transform.size.x,
            self.transform.size.y,
        )
    }
    fn contains(&self, point: Vec2) -> bool {
        self.rect().contains(point)
    }
}

struct GameState {
    player: Object,
    enemy: Object,
    ball: Object,
    score: (i32, i32),
    collision_detection: bool,
    player_lerp: f32,
    enemy_lerp: f32,
}

impl GameState {
    fn handle_ball(&mut self) {
        // bounce of top and bottom
        if self.ball.transform.pos.y < 1. {
            self.ball.velocity = vec2(self.ball.velocity.x, -self.ball.velocity.y);
            self.ball.transform.pos.y = 2.;
        } else if self.ball.transform.pos.y > screen_height() - self.ball.transform.size.y {
            self.ball.velocity = vec2(self.ball.velocity.x, -self.ball.velocity.y);
            self.ball.transform.pos.y = screen_height() - self.ball.transform.size.y - 1.;
        }
        // GOOOOL
        if self.ball.transform.pos.x < 0. {
            self.ball.transform.pos = vec2(screen_width() / 2., screen_height() / 2.0);
            self.ball.velocity = -self.ball.velocity;
            self.score.1 += 1;
        } else if self.ball.transform.pos.x > screen_width() {
            self.ball.transform.pos = vec2(screen_width() / 2., screen_height() / 2.0);
            self.ball.velocity = -self.ball.velocity;
            self.score.0 += 1;
        }
        // COLLISION
        if self.player.contains(self.ball.transform.pos) && self.collision_detection {
            self.ball.velocity = -self.ball.velocity;
            self.collision_detection = false;
        }
        if self.enemy.contains(self.ball.transform.pos) && self.collision_detection {
            self.ball.velocity = -self.ball.velocity;
            self.collision_detection = false
        }
        // if we pass the center turn the collision detection on again
        if !self.collision_detection
            && (self.ball.transform.pos.x - (screen_height() / 2.0)).abs() < 1.
        {
            self.collision_detection = true
        }
    }

    fn handle_enemy(&mut self) {
        let e_pos = self.enemy.transform.pos;
        GameState::handle_rackets(
            &mut self.enemy,
            self.ball.transform.pos.y > e_pos.y
                && self.ball.transform.pos.distance(e_pos) < screen_width() / 2.0,
            self.ball.transform.pos.y < e_pos.y
                && self.ball.transform.pos.distance(e_pos) < screen_width() / 2.0,
            &mut self.enemy_lerp,
        )
    }
    fn handle_player(&mut self) {
        GameState::handle_rackets(
            &mut self.player,
            is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
            is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
            &mut self.player_lerp,
        )
    }
    fn handle_rackets(racket: &mut Object, move_down: bool, move_up: bool, lerp_val: &mut f32) {
        if racket.transform.pos.y < 1. {
            racket.velocity = -racket.velocity / 2.;
            racket.transform.pos.y = 2.;
        } else if racket.transform.pos.y > screen_height() - racket.transform.size.y {
            racket.velocity = -racket.velocity / 2.;
            racket.transform.pos.y = screen_height() - racket.transform.size.y - 1.;
        } else {
            if move_down {
                racket.velocity += vec2(0., 2.);
                *lerp_val = 0.;
            } else if move_up {
                racket.velocity -= vec2(0., 2.);
                *lerp_val = 0.;
            } else {
                let sliperyness = 0.05;
                *lerp_val += get_frame_time() * sliperyness;
            }
            racket.velocity = racket.velocity.lerp(vec2(0., 0.), *lerp_val);
        }
    }
}
impl EventHandler for GameState {
    fn update(&mut self) {}

    fn draw(&mut self) {}
    fn resize_event(&mut self, _width: f32, _height: f32) {
        println!("resized");
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut player = Object {
        transform: Transform {
            pos: vec2(0., screen_height() / 2.0),
            size: vec2(10., 30.),
        },
        velocity: vec2(0., 0.0),
    };
    let mut enemy = Object {
        transform: Transform {
            pos: vec2(screen_width() - 10., screen_height() / 2.0),
            size: vec2(10., 30.),
        },
        velocity: vec2(0., 0.0),
    };
    let mut ball = Object {
        transform: Transform {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            size: vec2(4.0, 4.0),
        },
        velocity: vec2(-100., -100.0) * 1.3,
    };
    let mut game_state = GameState {
        player,
        enemy,
        ball,
        score: (0, 0),
        collision_detection: true,
        player_lerp: 0.,
        enemy_lerp: 0.,
    };
    loop {
        clear_background(BLACK);
        draw_line(
            screen_width() / 2.,
            0.,
            screen_width() / 2.,
            screen_height(),
            20.,
            WHITE,
        );
        draw_text(
            format!("{}     {}", game_state.score.0, game_state.score.1).as_str(),
            screen_width() / 2.0 - 55.,
            20.0,
            38.,
            WHITE,
        );
        draw_rectangle(
            game_state.player.transform.pos.x,
            game_state.player.transform.pos.y,
            game_state.player.transform.size.x,
            game_state.player.transform.size.y,
            WHITE,
        );
        draw_rectangle(
            game_state.enemy.transform.pos.x,
            game_state.enemy.transform.pos.y,
            game_state.enemy.transform.size.x,
            game_state.enemy.transform.size.y,
            WHITE,
        );
        draw_rectangle(
            game_state.ball.transform.pos.x,
            game_state.ball.transform.pos.y,
            game_state.ball.transform.size.x,
            game_state.ball.transform.size.y,
            WHITE,
        );
        game_state.handle_player();
        game_state.handle_enemy();
        game_state.handle_ball();
        // Physics
        game_state.ball.transform.pos += game_state.ball.velocity * get_frame_time();
        game_state.player.transform.pos += game_state.player.velocity * get_frame_time();
        game_state.enemy.transform.pos += game_state.enemy.velocity * get_frame_time();
        next_frame().await;
    }
}

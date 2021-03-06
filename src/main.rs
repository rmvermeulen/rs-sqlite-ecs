mod app;
mod components;
mod db;
mod system;
mod systems;

use crate::app::App;
use crate::db::Database;
use crate::system::System;
use crate::systems::collision::CollisionSystem;
use crate::systems::gravity::GravitySystem;
use crate::systems::movement::MovementSystem;

use anyhow::Result;
use minifb::{Window, WindowOptions};
use sqlite::Connection;
use std::time::Instant;

fn main() -> Result<()> {
    let (width, height) = (400, 400);
    let window = Window::new(
        "SqliteECS",
        width,
        height,
        WindowOptions {
            ..WindowOptions::default()
        },
    )?;

    let connection = Connection::open(":memory:")?;
    Database::initialize_tables(&connection)?;
    let mut app = App::new(window)?;

    let mut systems: Vec<Box<dyn System>> = vec![
        MovementSystem::new(&connection)?,
        GravitySystem::new(&connection)?,
        CollisionSystem::new(&connection)?,
    ];

    let mut delta: f64 = 0.;

    let game_start = Instant::now();
    loop {
        let frame_start = Instant::now();
        // game logic
        for system in &mut systems {
            system.tick(delta)?;
        }
        app.render(&connection)?;

        if game_start.elapsed().as_secs_f64() > 5. {
            break;
        }

        delta = frame_start.elapsed().as_secs_f64();
        // println!("fps: {} (delta: {})", 1.0 / delta, delta);
        app.fps = 1.0 / delta;
    }

    Ok(())
}

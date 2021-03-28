mod app;
mod components;
mod system;
mod systems;

use crate::app::App;
use crate::system::System;
use crate::systems::gravity::GravitySystem;
use crate::systems::movement::MovementSystem;
use crate::systems::print_position::PrintPositionSystem;
use anyhow::Result;
use minifb::Window;
use minifb::WindowOptions;
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
    let mut app = App::new(window, &connection)?;

    let boxed_movement_system = MovementSystem::new(&connection)?;

    let mut boxed_printer_system = PrintPositionSystem::new(&connection)?;
    boxed_printer_system.set_interval(1.0);

    let boxed_gravity_system = GravitySystem::new(&connection)?;

    let mut systems: Vec<Box<dyn System>> = vec![
        boxed_movement_system,
        boxed_printer_system,
        boxed_gravity_system,
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
    }

    Ok(())
}

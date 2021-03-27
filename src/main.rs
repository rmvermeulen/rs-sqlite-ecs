mod app;
mod system;
mod systems;

use crate::app::App;
use crate::system::System;
use crate::systems::gravity::GravitySystem;
use crate::systems::movement::MovementSystem;
use crate::systems::print_position::PrintPositionSystem;
use minifb::{Window, WindowOptions};
use sqlite::Result;

fn main() -> Result<()> {
    let app = App::new("SqliteECS", (400, 400))?;

    let boxed_movement_system = MovementSystem::new(&app)?;

    let mut boxed_printer_system = PrintPositionSystem::new(&app)?;
    boxed_printer_system.set_interval(1.0);

    let boxed_gravity_system = GravitySystem::new(&app)?;

    let mut systems: Vec<Box<dyn System>> = vec![
        boxed_movement_system,
        boxed_printer_system,
        boxed_gravity_system,
    ];

    let framerate = 25;
    let mut count = 0;
    let mut delta: f64 = 0.;
    let mut fps = fps_clock::FpsClock::new(framerate);

    loop {
        // game logic
        for system in &mut systems {
            system.tick(delta)?;
        }

        count += 1;
        // update timer
        delta = fps.tick() as f64 / 10e8;
        if count > 5 * framerate {
            break;
        }
    }

    Ok(())
}

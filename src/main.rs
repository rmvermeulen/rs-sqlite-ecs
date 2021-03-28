mod app;
mod system;
mod systems;

use crate::app::App;
use crate::system::System;
use crate::systems::gravity::GravitySystem;
use crate::systems::movement::MovementSystem;
use crate::systems::print_position::PrintPositionSystem;
use crate::systems::renderer::RenderSystem;
use anyhow::Result;
use std::rc::Rc;

fn main() -> Result<()> {
    let mut app = App::new("SqliteECS", (400, 400))?;

    let boxed_movement_system = MovementSystem::new(&app.db)?;

    let mut boxed_printer_system = PrintPositionSystem::new(&app.db)?;
    boxed_printer_system.set_interval(1.0);

    let boxed_gravity_system = GravitySystem::new(&app.db)?;

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
        app.render()?;

        count += 1;
        // update timer
        delta = fps.tick() as f64 / 10e8;
        if count > 8 * framerate {
            break;
        }
    }

    Ok(())
}

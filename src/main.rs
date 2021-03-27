mod app;
mod system;
mod systems;

use crate::app::App;
use crate::system::System;
use crate::systems::movement::MovementSystem;
use crate::systems::print_position::PrintPositionSystem;
use sqlite::{Connection, Result, Statement};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

struct PreparedStatement<'conn>(Statement<'conn>);

impl<'conn> PreparedStatement<'conn> {
    pub fn new<'a>(conn: &'a Connection, sql: &str) -> Result<PreparedStatement<'a>> {
        Ok(PreparedStatement(conn.prepare(sql)?))
    }
}

fn main() -> Result<()> {
    let app = App::new()?;
    let mut boxed_movement_system = MovementSystem::new(&app)?;
    let mut boxed_printer_system = PrintPositionSystem::new(&app)?;
    boxed_printer_system.set_interval(0.25);
    let mut systems: Vec<Box<dyn System>> = vec![boxed_movement_system, boxed_printer_system];

    let framerate = 25;
    let mut count = 0;
    let mut delta: f64 = 0.;
    let mut fps = fps_clock::FpsClock::new(framerate);

    loop {
        let is_whole_second = count % framerate == 0;

        // game logic
        for system in &mut systems {
            system.tick(delta)?;
        }

        if is_whole_second {
            println!("a second passed");
            println!("delta: {:?}", delta);
            println!("efps: {:?}/{:?}", (1.0 / delta).trunc(), framerate);
        }
        // println!("{} {} (fps: {})", count / framerate, delta, 1.0 / delta);

        count += 1;
        // update timer
        delta = fps.tick() as f64 / 10e8;
        if count > 5 * framerate {
            break;
        }
    }

    Ok(())
}

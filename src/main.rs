#[macro_use]
use impls::impls;

use diesel::backend::SupportsReturningClause;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    dotenv().ok();

    assert!(impls!(diesel::sqlite::Sqlite: SupportsReturningClause));

    // let conn = Connection::open_in_memory()?;

    // conn.execute_batch(
    //     "
    //     BEGIN;

    //     CREATE TABLE entity (
    //         id          INTEGER PRIMARY KEY
    //     );
    //     CREATE TABLE position (
    //         id          INTEGER,
    //         x           FLOAT DEFAULT 0,
    //         y           FLOAT DEFAULT 0,

    //         FOREIGN KEY(id) REFERENCES entity(id)
    //     );
    //     CREATE TABLE velocity (
    //         id          INTEGER,
    //         x           FLOAT DEFAULT 0,
    //         y           FLOAT DEFAULT 0,

    //         FOREIGN KEY(id) REFERENCES entity(id)
    //     );

    //     COMMIT;",
    // )?;

    // let id: i32 = conn.query_row(
    //     "INSERT INTO entity DEFAULT VALUES RETURNING *;",
    //     NO_PARAMS,
    //     |row| row.get(0),
    // )?;

    // // .and_then(|id: i32| {
    // //     conn.execute("INSERT INTO position VALUES (?, 100, 100)", params![id])
    // //         .and(conn.execute("INSERT INTO velocity VALUES (?, 0, -1)", params![id]))
    // // })?;

    // // conn.execute_batch(
    // //     "
    // //     BEGIN;

    // //     REPLACE INTO position VALUES (:id, 100, 100);

    // //     REPLACE INTO velocity VALUES (:id, 0, -10);

    // //     COMMIT;",
    // //     named_params! {":id": entity as i32},
    // // )?;

    // let mut velocity_system = conn.prepare(
    //     "
    //     UPDATE position AS p SET
    //         x = p.x + (v.x * :delta),
    //         y = p.y + (v.y * :delta)
    //     FROM velocity v WHERE p.id = v.id
    // ",
    // )?;

    // let mut print_position_system = conn.prepare(
    //     "
    //     SELECT p.id, p.x, p.y, v.x, v.y
    //     FROM position p
    //     JOIN velocity v
    //     ON p.id = v.id
    //     ",
    // )?;

    // let framerate = 25;
    // let mut count = 0;
    // let mut delta: f64 = 0.;
    // let mut fps = fps_clock::FpsClock::new(framerate);

    // loop {
    //     let is_whole_second = count % framerate == 0;

    //     // game logic
    //     velocity_system.execute_named(named_params! {":delta":delta})?;

    //     if is_whole_second {
    //         let mut positions = print_position_system.query(NO_PARAMS)?;
    //         while let Some(pos) = positions.next()? {
    //             println!(
    //                 "{:?} {{ pos: ({:?}, {:?}), vel: ({:?}, {:?}) }}",
    //                 pos.get::<usize, i32>(0)?,
    //                 pos.get::<usize, f64>(1)?,
    //                 pos.get::<usize, f64>(2)?,
    //                 pos.get::<usize, f64>(3)?,
    //                 pos.get::<usize, f64>(4)?
    //             );
    //         }
    //     }
    //     // println!("{} {} (fps: {})", count / framerate, delta, 1.0 / delta);

    //     count += 1;
    //     if is_whole_second {
    //         println!("a second passed");
    //         println!("delta: {:?}", delta);
    //         println!("efps: {:?}/{:?}", (1.0 / delta).trunc(), framerate);
    //     }
    //     // update timer
    //     delta = fps.tick() as f64 / 10e8;
    //     if count > 5 * framerate {
    //         break;
    //     }
    // }

    Ok(())
}

use rusqlite::{named_params, params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

use sql_builder::SqlBuilder;

fn main() -> Result<()> {
    let sql = SqlBuilder::select_from("company")
        .field("id")
        .field("name")
        .and_where_gt("salary", 25_000)
        .sql();

    match sql {
        Ok(sql) => println!("{}", sql),
        Err(e) => println!("Error: {}", e),
    }

    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "
        BEGIN;

        CREATE TABLE entity (
            id          INTEGER PRIMARY KEY
        );
        CREATE TABLE position (
            id          INTEGER,
            x           FLOAT DEFAULT 0,
            y           FLOAT DEFAULT 0,

            FOREIGN KEY(id) REFERENCES entity(id)
        );
        CREATE TABLE velocity (
            id          INTEGER,
            x           FLOAT DEFAULT 0,
            y           FLOAT DEFAULT 0,

            FOREIGN KEY(id) REFERENCES entity(id)
        );

        COMMIT;",
    )?;

    conn.execute_batch(
        "
        BEGIN;

        REPLACE INTO entity DEFAULT VALUES;

        REPLACE INTO position VALUES (:id, 100, 100);

        REPLACE INTO velocity VALUES (:id, 0, -10);

        COMMIT;",
    )?;

    let mut velocity_system = conn.prepare(
        "
        UPDATE position AS p SET
            x = p.x + (v.x * :delta),
            y = p.y + (v.y * :delta)
        FROM velocity v WHERE p.id = v.id
    ",
    )?;

    let mut print_position_system = conn.prepare(
        "
        SELECT p.id, p.x, p.y, v.x, v.y
        FROM position p
        JOIN velocity v
        ON p.id = v.id
        ",
    )?;

    let framerate = 25;
    let mut count = 0;
    let mut delta: f64 = 0.;
    let mut fps = fps_clock::FpsClock::new(framerate);

    loop {
        let is_whole_second = count % framerate == 0;

        // game logic
        velocity_system.execute_named(named_params! {":delta":delta})?;

        if is_whole_second {
            let mut positions = print_position_system.query(NO_PARAMS)?;
            while let Some(pos) = positions.next()? {
                println!(
                    "{:?} {{ pos: ({:?}, {:?}), vel: ({:?}, {:?}) }}",
                    pos.get::<usize, i32>(0)?,
                    pos.get::<usize, f64>(1)?,
                    pos.get::<usize, f64>(2)?,
                    pos.get::<usize, f64>(3)?,
                    pos.get::<usize, f64>(4)?
                );
            }
        }
        // println!("{} {} (fps: {})", count / framerate, delta, 1.0 / delta);

        count += 1;
        if is_whole_second {
            println!("a second passed");
            println!("delta: {:?}", delta);
            println!("efps: {:?}/{:?}", (1.0 / delta).trunc(), framerate);
        }
        // update timer
        delta = fps.tick() as f64 / 10e8;
        if count > 5 * framerate {
            break;
        }
    }

    Ok(())
}

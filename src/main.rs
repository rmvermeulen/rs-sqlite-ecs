use rusqlite::{named_params, params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  x               FLOAT DEFAULT 0.0,
                  y               FLOAT DEFAULT 0.0,
                  vx              FLOAT DEFAULT 0.0,
                  vy              FLOAT DEFAULT 0.0)",
        params![],
    )?;
    let person = Person {
        id: 0,
        name: "Steven".to_string(),
        position: (0., 0.),
        velocity: (0., 0.),
    };
    conn.execute(
        "INSERT INTO person (name, x, y, vx) VALUES (?1, ?2, ?3, ?4)",
        params![person.name, person.position.0, person.position.1, 10.],
    )?;

    let framerate = 30;
    let mut updater = conn.prepare("UPDATE person SET x = x + vx * :delta, y = y + vy * :delta")?;
    let mut stmt = conn.prepare("SELECT id, name, x, y, vx, vy FROM person")?;
    let mut count = 0;
    let mut delta: f64 = 0.;
    let mut fps = fps_clock::FpsClock::new(framerate);
    loop {
        let updated = updater.execute_named(named_params! {":delta": delta })?;
        let persons: Vec<_> = stmt
            .query_map(params![], |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    position: (row.get(2)?, row.get(3)?),
                    velocity: (row.get(4)?, row.get(5)?),
                })
            })?
            .map(|person| {
                let person = person.unwrap();

                format!("{}@{:?}", person.name, person.position)
            })
            .collect();

        // game logic
        println!("{} {} (fps: {}) {:?}", count, delta, 1.0 / delta, persons);
        count += 1;
        // update timer
        delta = fps.tick() as f64 / 10e8;
        if count > 60 * framerate {
            break;
        }
    }

    Ok(())
}

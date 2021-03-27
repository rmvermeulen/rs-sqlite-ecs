use rusqlite::{named_params, params, Connection, Result, Statement, NO_PARAMS};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

struct App {
    db: Connection,
}

impl App {
    fn new() -> Result<Self> {
        let db = Connection::open_in_memory()?;
        db.execute_batch(
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

        db.execute_batch(
            "
        BEGIN;

        INSERT INTO entity DEFAULT VALUES;

        REPLACE INTO position VALUES (1, 100, 100);

        REPLACE INTO velocity VALUES (1, 0, -10);

        COMMIT;",
        )?;

        {
            let mut get_ids = db.prepare("select * from entity")?;
            let mut rows = get_ids.query(NO_PARAMS)?;
            println!("iterating ids...");
            let mut ids = Vec::new();
            while let Some(row) = rows.next()? {
                ids.push(row.get::<usize, i32>(0)?);
            }
            println!("got ids: {:?}", ids);
            println!("done");
        }
        Ok(Self { db })
    }
}

trait System<'a>: Sized {
    fn new(app: &'a App) -> Result<Self>;
    fn tick(&mut self, delta: f64);
}

struct PreparedStatement<'conn> {
    pub statement: Statement<'conn>,
}

impl<'conn> PreparedStatement<'conn> {
    pub fn new<'a>(conn: &'a Connection, sql: &str) -> Result<PreparedStatement<'a>> {
        Ok(PreparedStatement {
            statement: conn.prepare(sql)?,
        })
    }
}

struct VelocitySystem<'a> {
    sql: PreparedStatement<'a>,
}

impl<'a> System<'a> for VelocitySystem<'a> {
    fn new(app: &'a App) -> Result<Self> {
        let statement = PreparedStatement::new(
            &app.db,
            "UPDATE position AS p SET
                x = p.x + (v.x * :delta),
                y = p.y + (v.y * :delta)
            
                FROM velocity v WHERE p.id = v.id",
        )?;
        Ok(VelocitySystem { sql: statement })
    }
    fn tick(&mut self, delta: f64) {
        self.sql
            .statement
            .execute_named(named_params! {":delta":delta})
            .unwrap();
    }
}

struct PrintPositionSystem<'a> {
    sql: PreparedStatement<'a>,
}

impl<'a> System<'a> for PrintPositionSystem<'a> {
    fn new(app: &'a App) -> Result<Self> {
        let statement = PreparedStatement::new(
            &app.db,
            "
            SELECT e.id, p.x, p.y, v.x, v.y
            FROM entity e
            JOIN position p ON p.id = e.id
            JOIN velocity v ON v.id = e.id",
        )?;
        Ok(PrintPositionSystem { sql: statement })
    }
    fn tick(&mut self, delta: f64) {
        let mut positions = self.sql.statement.query(NO_PARAMS).unwrap();
        while let Some(pos) = positions.next().unwrap() {
            println!(
                "{:?} {{ pos: ({:?}, {:?}), vel: ({:?}, {:?}) }}",
                pos.get::<usize, i32>(0).unwrap(),
                pos.get::<usize, f64>(1).unwrap(),
                pos.get::<usize, f64>(2).unwrap(),
                pos.get::<usize, f64>(3).unwrap(),
                pos.get::<usize, f64>(4).unwrap()
            );
        }
    }
}

fn main() -> Result<()> {
    let app = App::new()?;
    let mut velocity = VelocitySystem::new(&app)?;
    let mut printer = PrintPositionSystem::new(&app)?;

    let framerate = 25;
    let mut count = 0;
    let mut delta: f64 = 0.;
    let mut fps = fps_clock::FpsClock::new(framerate);

    loop {
        let is_whole_second = count % framerate == 0;

        // game logic
        velocity.tick(delta);
        printer.tick(delta);

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

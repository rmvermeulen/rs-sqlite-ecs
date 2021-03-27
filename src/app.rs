use sqlite::{Connection, Result};

pub struct App {
  pub db: Connection,
}

impl App {
  pub fn new() -> Result<Self> {
    let db = Connection::open(":memory:")?;
    db.execute(
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
        CREATE TABLE gravity (
            id          INTEGER,
            amount      FLOAT DEFAULT 98.0,

            FOREIGN KEY(id) REFERENCES entity(id)
        );

        COMMIT;",
    )?;

    db.execute(
      "
        BEGIN;

        INSERT INTO entity DEFAULT VALUES;
        INSERT INTO position VALUES (1, 100, 100);
        INSERT INTO velocity VALUES (1, 0, -10);
        INSERT INTO gravity VALUES (1, 98);

        COMMIT;",
    )?;

    Ok(Self { db })
  }
}

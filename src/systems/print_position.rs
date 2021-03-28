use crate::app::App;
use crate::system::System;
use anyhow::Result;
use sqlite::{Connection, State, Statement};

pub struct PrintPositionSystem<'a> {
  sql: Statement<'a>,
  interval: f64,
  remaining: f64,
}
impl PrintPositionSystem<'_> {
  pub fn set_interval(&mut self, ms: f64) {
    self.interval = ms;
    self.remaining = ms;
  }
}
impl<'a> System<'a> for PrintPositionSystem<'a> {
  fn new(connection: &'a Connection) -> Result<Box<Self>> {
    let statement = connection.prepare(
      "
            SELECT e.id, p.x, p.y, v.x, v.y
            FROM entity e
            JOIN position p ON p.id = e.id
            JOIN velocity v ON v.id = e.id",
    )?;
    Ok(Box::new(PrintPositionSystem {
      sql: statement,
      interval: 0.,
      remaining: 0.,
    }))
  }
  fn tick(&mut self, delta: f64) -> Result<()> {
    self.remaining = if self.remaining - delta < 0. {
      0.
    } else {
      self.remaining - delta
    };
    if self.remaining > 0. {
      return Ok(());
    }

    // reset timer
    self.remaining = self.interval;
    // reset prepared statement
    self.sql.reset()?;
    while let State::Row = self.sql.next()? {
      println!(
        "{:?} {{ pos: ({:?}, {:?}), vel: ({:?}, {:?}) }}",
        self.sql.read::<i64>(0).unwrap(),
        self.sql.read::<f64>(1).unwrap(),
        self.sql.read::<f64>(2).unwrap(),
        self.sql.read::<f64>(3).unwrap(),
        self.sql.read::<f64>(4).unwrap()
      );
    }

    Ok(())
  }
}

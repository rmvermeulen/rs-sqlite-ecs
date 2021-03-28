use crate::app::App;
use crate::system::System;
use anyhow::Result;
use sqlite::{Connection, State, Statement};

pub struct MovementSystem<'a> {
  sql: Statement<'a>,
}
impl<'a> System<'a> for MovementSystem<'a> {
  fn new(connection: &'a Connection) -> Result<Box<Self>> {
    let statement = connection.prepare(
      "
      UPDATE position AS p
      SET x = p.x + (v.x * :delta),
          y = p.y + (v.y * :delta)
      FROM velocity v WHERE p.id = v.id",
    )?;
    Ok(Box::new(MovementSystem { sql: statement }))
  }
  fn tick(&mut self, delta: f64) -> Result<()> {
    self.sql.reset()?;
    self.sql.bind_by_name(":delta", delta)?;

    assert!(self.sql.next()? == State::Done, "Completes in one step");

    Ok(())
  }
}

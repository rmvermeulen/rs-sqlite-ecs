use crate::app::App;
use crate::system::System;
use sqlite::{Result, State, Statement};

pub struct MovementSystem<'a> {
  sql: Statement<'a>,
}
impl<'a> System<'a> for MovementSystem<'a> {
  fn new(app: &'a App) -> Result<Box<Self>> {
    let statement = app.db.prepare(
      "UPDATE position AS p SET
                x = p.x + (v.x * :delta),
                y = p.y + (v.y * :delta)
            
                FROM velocity v WHERE p.id = v.id",
    )?;
    Ok(Box::new(MovementSystem { sql: statement }))
  }
  fn tick(&mut self, delta: f64) -> Result<()> {
    self.sql.reset()?;
    self.sql.bind_by_name(":delta", delta)?;

    while let State::Row = self.sql.next()? {}

    Ok(())
  }
}

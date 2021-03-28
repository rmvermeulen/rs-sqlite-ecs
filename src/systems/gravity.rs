use crate::app::App;
use crate::system::System;
use anyhow::Result;
use sqlite::{Connection, State, Statement};

pub struct GravitySystem<'a> {
    sql: Statement<'a>,
}

impl<'a> System<'a> for GravitySystem<'a> {
    fn new(connection: &'a Connection) -> Result<Box<Self>> {
        let statement = connection.prepare(
            "
      UPDATE velocity AS v
      SET y = v.y - (g.amount * :delta)
      FROM gravity g WHERE g.id = v.id
        ",
        )?;
        Ok(Box::new(GravitySystem { sql: statement }))
    }
    fn tick(&mut self, delta: f64) -> Result<()> {
        self.sql.reset()?;
        self.sql.bind_by_name(":delta", delta)?;

        assert!(self.sql.next()? == State::Done, "Completes in one step");

        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Database;
  #[derive(Debug, PartialEq)]
  struct Position {
    id: i32,
    x: f64,
    y: f64,
  }
  fn get_positions(connection: &Connection) -> Result<Vec<Position>> {
    let mut results = Vec::new();

    connection.iterate("select id, x, y from position", |pairs| {
      let mut mPos = None;
      for &(column, value) in pairs.iter() {
        mPos = match column {
          "id" => Some(Position {
            id: value.unwrap().parse::<i32>().unwrap(),
            x: 0.,
            y: 0.,
          }),
          "x" => mPos.map(|mut pos| {
            pos.x = value.unwrap().parse::<f64>().unwrap();
            pos
          }),

          "y" => {
            mPos.map(|mut pos| {
              pos.y = value.unwrap().parse::<f64>().unwrap();
              results.push(pos);
            });
            None
          }
          _ => mPos,
        }
      }
      true
    })?;

    Ok(results)
  }
  #[test]
  fn update_position_with_velocity() -> Result<()> {
    let connection = Connection::open(":memory:")?;
    Database::initialize_tables(&connection)?;

    // set velocity on component
    connection.execute("update velocity set x = 50, y = 25 where id = 2;")?;

    assert_eq!(
      get_positions(&connection)?,
      vec![
        Position {
          id: 1,
          x: 200.0,
          y: 400.0
        },
        Position {
          id: 2,
          x: 100.0,
          y: 100.0
        },
        Position {
          id: 3,
          x: 200.0,
          y: 100.0
        },
      ]
    );

    let mut system = MovementSystem::new(&connection)?;

    system.tick(1.0)?;

    let mut results = Vec::new();
    connection.iterate("select x, y from position", |pairs| {
      for &(column, value) in pairs.iter() {
        match value.unwrap().parse::<f64>() {
          Ok(n) => results.push((String::from(column), n)),
          _ => {}
        }
      }
      true
    })?;

    assert_eq!(
      get_positions(&connection)?,
      vec![
        Position {
          id: 1,
          x: 200.0,
          y: 400.0
        },
        Position {
          id: 2,
          x: 100.0 + 50.0,
          y: 100.0 + 25.0
        },
        Position {
          id: 3,
          x: 200.0,
          y: 100.0
        },
      ]
    );
    Ok(())
  }
}

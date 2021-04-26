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
  struct TestData {
    id: i32,
    x: f64,
    y: f64,
  }
  #[test]
  fn update_position_with_velocity() -> Result<()> {
    let connection = Connection::open(":memory:")?;
    Database::initialize_tables(&connection)?;

    // set velocity on component
    connection.execute("update velocity set x = 50, y = 25 where id = 2;")?;

    assert_eq!(
      get_entities_positions_helper(&connection)?,
      vec![
        TestData {
          id: 1,
          x: 200.0,
          y: 400.0
        },
        TestData {
          id: 2,
          x: 100.0,
          y: 100.0
        },
        TestData {
          id: 3,
          x: 200.0,
          y: 100.0
        },
      ]
    );

    let mut system = MovementSystem::new(&connection)?;

    system.tick(1.0)?;

    assert_eq!(
      get_entities_positions_helper(&connection)?,
      vec![
        TestData {
          id: 1,
          x: 200.0,
          y: 400.0
        },
        TestData {
          id: 2,
          x: 100.0 + 50.0,
          y: 100.0 + 25.0
        },
        TestData {
          id: 3,
          x: 200.0,
          y: 100.0
        },
      ]
    );
    Ok(())
  }
  fn get_entities_positions_helper(connection: &Connection) -> Result<Vec<TestData>> {
    let mut results = Vec::new();

    connection.iterate("select id, x, y from position", |pairs| {
      let mut current_data = None;
      for &(column, value) in pairs.iter() {
        current_data = match column {
          "id" => Some(TestData {
            id: value.unwrap().parse::<i32>().unwrap(),
            x: 0.,
            y: 0.,
          }),
          "x" => current_data.map(|mut pos| {
            pos.x = value.unwrap().parse::<f64>().unwrap();
            pos
          }),

          "y" => {
            current_data.map(|mut pos| {
              pos.y = value.unwrap().parse::<f64>().unwrap();
              results.push(pos);
            });
            None
          }
          _ => current_data,
        }
      }
      true
    })?;

    Ok(results)
  }
}

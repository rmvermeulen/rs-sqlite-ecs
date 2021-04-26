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
      SET y = v.y + (g.amount * :delta)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Database;
    #[derive(Debug, PartialEq)]
    struct VelocityData {
        id: i32,
        x: f64,
        y: f64,
    }
    #[test]
    fn update_velocity_with_gravity() -> Result<()> {
        let connection = Connection::open(":memory:")?;
        Database::initialize_tables(&connection)?;

        // set gravity on component
        connection.execute(
            "
            update gravity set amount = 25 where id = 2;
            update gravity set amount = 0 where id = 3;
            ",
        )?;
        let values = get_entities_velocity_helper(&connection)?;
        println!("before {:?}", values);
        assert_eq!(
            values,
            vec![
                VelocityData {
                    id: 2,
                    x: 0.0,
                    y: 0.0
                },
                VelocityData {
                    id: 3,
                    x: 0.0,
                    y: 0.0
                },
            ]
        );

        println!("passed this assert");

        let mut system = GravitySystem::new(&connection)?;

        system.tick(1.0)?;

        println!("passed the system.tick()");

        let values = get_entities_velocity_helper(&connection)?;
        println!("after {:?}", values);
        assert_eq!(
            values,
            vec![
                VelocityData {
                    id: 2,
                    x: 0.0,
                    y: 25.0
                },
                VelocityData {
                    id: 3,
                    x: 0.0,
                    y: 0.0
                },
            ]
        );
        Ok(())
    }
    fn get_entities_velocity_helper(connection: &Connection) -> Result<Vec<VelocityData>> {
        let mut results = Vec::new();

        connection.iterate("select id, x, y from velocity", |pairs| {
            let mut current_data = None;
            for &(column, value) in pairs.iter() {
                current_data = match column {
                    "id" => Some(VelocityData {
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

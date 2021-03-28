use anyhow::Result;
use sqlite::Connection;

pub trait System<'a> {
  fn new(connection: &'a Connection) -> Result<Box<Self>>
  where
    Self: Sized;
  fn tick(&mut self, delta: f64) -> Result<()>;
}

use crate::app::App;
use sqlite::Result;

pub trait System<'a> {
  fn new(app: &'a App) -> Result<Box<Self>>
  where
    Self: Sized;
  fn tick(&mut self, delta: f64) -> Result<()>;
}

use anyhow::{anyhow, Result};
use sqlite::Value;
use sqlite::{Connection, State};

pub enum Components {
  Position { x: f64, y: f64 },
  Velocity { x: f64, y: f64 },
  Gravity(f64),
  Graphics { shape: String, color: String },
}
pub fn entity_add_component(
  connection: &Connection,
  entity: i64,
  component: Components,
) -> Result<Components> {
  use Components::*;
  match component {
    Position { x, y } => {
      let mut s = connection.prepare("INSERT INTO position VALUES (:id, :x, :y)")?;
      s.bind_by_name(":id", entity)?;
      s.bind_by_name(":x", x)?;
      s.bind_by_name(":y", y)?;
      while let State::Row = s.next()? {}
    }
    Velocity { x, y } => {
      let mut s = connection.prepare("INSERT INTO velocity VALUES (:id, :x, :y)")?;
      s.bind_by_name(":id", entity)?;
      s.bind_by_name(":x", x)?;
      s.bind_by_name(":y", y)?;
      while let State::Row = s.next()? {}
    }
    Gravity(amount) => {
      let mut s = connection.prepare("INSERT INTO gravity VALUES (:id, :amount)")?;
      s.bind_by_name(":id", entity)?;
      s.bind_by_name(":amount", amount)?;
      while let State::Row = s.next()? {}
    }
    Graphics {
      ref shape,
      ref color,
    } => {
      let mut s = connection.prepare("INSERT INTO graphics VALUES (:id, :shape, :color)")?;
      s.bind_by_name(":id", entity)?;
      s.bind_by_name(":shape", &Value::String(shape.clone()))?;
      s.bind_by_name(":color", &Value::String(color.clone()))?;
      while let State::Row = s.next()? {}
    }
  };
  Ok(component)
}

// pub fn entity_remove_component() -> Result<()> { }

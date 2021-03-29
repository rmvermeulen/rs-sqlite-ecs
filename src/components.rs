use anyhow::{anyhow, Result};
use sqlite::Value;
use sqlite::{Connection, State};

pub enum Component {
  Position {
    x: f64,
    y: f64,
  },
  Velocity {
    x: f64,
    y: f64,
  },
  Gravity(f64),
  Graphics {
    width: f64,
    height: f64,
    color: String,
  },
}
pub fn entity_add_component(
  connection: &Connection,
  entity: i64,
  component: Component,
) -> Result<Component> {
  use Component::*;
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
      width,
      height,
      ref color,
    } => {
      let mut s =
        connection.prepare("INSERT INTO graphics VALUES (:id, :width, :height, :color)")?;
      s.bind_by_name(":id", entity)?;
      s.bind_by_name(":width", width)?;
      s.bind_by_name(":height", height)?;
      s.bind_by_name(":color", &Value::String(color.clone()))?;
      while let State::Row = s.next()? {}
    }
  };
  Ok(component)
}

pub struct Builder<'a> {
  connection: &'a Connection,
  entity: Option<i64>,
  components: Vec<Component>,
}

impl<'a> Builder<'a> {
  pub fn new(connection: &'a Connection) -> Builder {
    Builder {
      connection,
      entity: None,
      components: Vec::new(),
    }
  }
  pub fn set_entity(&mut self, entity: i64) -> &mut Self {
    self.entity = if entity < 0 { None } else { Some(entity) };
    self
  }
  pub fn add_component(&mut self, component: Component) -> Result<&mut Self> {
    use Component::*;
    for c in &self.components {
      match (c, &component) {
        (Position { .. }, Position { .. }) => {
          return Err(anyhow!("Already have Position component"))
        }
        (Velocity { .. }, Velocity { .. }) => {
          return Err(anyhow!("Already have Velocity component"))
        }
        (Gravity { .. }, Gravity { .. }) => return Err(anyhow!("Already have Gravity component")),
        (Graphics { .. }, Graphics { .. }) => {
          return Err(anyhow!("Already have Graphics component"))
        }
        _ => continue,
      }
    }
    self.components.push(component);
    Ok(self)
  }
  pub fn finish(&mut self) -> Result<()> {
    if self.entity == None {
      println!("builder: creating entity...");
      self
        .connection
        .execute("INSERT INTO entity DEFAULT VALUES")?;
      let mut s = self
        .connection
        .prepare("SELECT * FROM entity ORDER BY id DESC LIMIT 1")?;

      while let State::Row = s.next()? {
        let id = s.read::<i64>(0)?;
        println!("builder: created entity {}", id);
        self.entity = Some(id);
      }
    }
    match self.entity {
      Some(entity) => {
        while let Some(component) = self.components.pop() {
          entity_add_component(self.connection, entity, component)?;
        }
        Ok(())
      }
      None => Err(anyhow!("Invalid entity")),
    }
  }
}

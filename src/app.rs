use crate::components::{entity_add_component, Components};

use anyhow::Result;
use minifb::Window;
use raqote::DrawOptions;
use raqote::DrawTarget;
use raqote::PathBuilder;
use raqote::SolidSource;
use raqote::Source;
use sqlite::{Connection, State};

#[derive(Debug)]
struct RenderData {
  shape: String,
  color: String,
  x: f64,
  y: f64,
}

pub struct App {
  window: Window,
  target: DrawTarget,
}

impl App {
  pub fn new(window: Window, connection: &Connection) -> Result<Self> {
    connection.execute(
      "
        BEGIN;

        CREATE TABLE entity (
            id          INTEGER PRIMARY KEY
        );
        CREATE TABLE position (
            id          INTEGER,
            x           FLOAT DEFAULT 0,
            y           FLOAT DEFAULT 0,

            FOREIGN KEY(id) REFERENCES entity(id)
        );
        CREATE TABLE velocity (
            id          INTEGER,
            x           FLOAT DEFAULT 0,
            y           FLOAT DEFAULT 0,

            FOREIGN KEY(id) REFERENCES entity(id)
        );
        CREATE TABLE gravity (
          id          INTEGER,
          amount      FLOAT DEFAULT 98.0,

          FOREIGN KEY(id) REFERENCES entity(id)
        );
        CREATE TABLE graphics (
          id         INTEGER,
          shape      TEXT,
          color      TEXT,

          FOREIGN KEY(id) REFERENCES entity(id)
        );

        COMMIT;",
    )?;

    connection.execute(
      "
        BEGIN;

        INSERT INTO entity DEFAULT VALUES;

        COMMIT;",
    )?;

    entity_add_component(connection, 1, Components::Position { x: 100., y: 100. })?;
    entity_add_component(connection, 1, Components::Velocity { x: 0., y: 0. })?;
    entity_add_component(connection, 1, Components::Gravity(9.8))?;
    entity_add_component(
      connection,
      1,
      Components::Graphics {
        shape: String::from("rect"),
        color: String::from("red"),
      },
    )?;

    let (width, height) = window.get_size();

    Ok(App {
      window,
      target: DrawTarget::new(width as i32, height as i32),
    })
  }
  pub fn render(&mut self, connection: &Connection) -> Result<()> {
    let mut sql = connection.prepare(
      "
      SELECT g.shape, g.color, p.x, p.y
      FROM entity e
      JOIN graphics g ON g.id = e.id
      JOIN position p ON p.id = e.id
      ",
    )?;

    while let State::Row = sql.next()? {
      self.render_entity(RenderData {
        shape: sql.read::<String>(0)?,
        color: sql.read::<String>(1)?,
        x: sql.read::<f64>(2)?,
        y: sql.read::<f64>(3)?,
      });
    }
    let (w, h) = self.window.get_size();
    self
      .window
      .update_with_buffer(self.target.get_data(), w, h)?;

    Ok(())
  }
  fn render_entity(&mut self, data: RenderData) {
    let bg_color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    let rect_color = Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0));
    self.target.clear(bg_color);

    let mut pb = PathBuilder::new();
    pb.rect((data.x - 16.) as f32, (data.y - 16.) as f32, 32., 32.);
    let path = pb.finish();
    self.target.fill(&path, &rect_color, &DrawOptions::new());
  }
}

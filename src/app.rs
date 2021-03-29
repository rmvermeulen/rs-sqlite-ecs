use crate::components::{Builder, Component};

use anyhow::Result;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
use sqlite::{Connection, State};

#[derive(Debug)]
struct RenderData {
  color: String,
  x: f64,
  y: f64,
  width: f64,
  height: f64,
}

pub struct App {
  pub fps: f64,
  window: Window,
  target: DrawTarget,
}

fn color(r: u8, g: u8, b: u8, a: u8) -> SolidSource {
  SolidSource::from_unpremultiplied_argb(a, r, g, b)
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
          amount      FLOAT DEFAULT 0.0,

          FOREIGN KEY(id) REFERENCES entity(id)
        );
        CREATE TABLE graphics (
          id         INTEGER,
          width      FLOAT DEFAULT 8.0,
          height     FLOAT DEFAULT 8.0,
          color      TEXT,

          FOREIGN KEY(id) REFERENCES entity(id)
        );

        COMMIT;",
    )?;

    connection.execute(
      "
        BEGIN;

        -- INSERT INTO entity DEFAULT VALUES;
        -- INSERT INTO entity DEFAULT VALUES;

        COMMIT;",
    )?;

    Builder::new(connection)
      .add_component(Component::Position { x: 200., y: 400. })?
      .add_component(Component::Graphics {
        width: 128.,
        height: 8.,
        color: String::from("green"),
      })?
      .finish()?;

    Builder::new(connection)
      .add_component(Component::Position { x: 100., y: 100. })?
      .add_component(Component::Velocity { x: 0., y: 0. })?
      .add_component(Component::Gravity(100.))?
      .add_component(Component::Graphics {
        width: 32.,
        height: 32.,
        color: String::from("red"),
      })?
      .finish()?;
    Builder::new(connection)
      .add_component(Component::Position { x: 200., y: 100. })?
      .add_component(Component::Velocity { x: 0., y: 0. })?
      .add_component(Component::Gravity(100.))?
      .add_component(Component::Graphics {
        width: 32.,
        height: 32.,
        color: String::from("blue"),
      })?
      .finish()?;

    let (width, height) = window.get_size();

    Ok(App {
      fps: 0.,
      window,
      target: DrawTarget::new(width as i32, height as i32),
    })
  }
  pub fn render(&mut self, connection: &Connection) -> Result<()> {
    let bg_color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    self.target.clear(bg_color);

    let mut sql = connection.prepare(
      "
      SELECT p.x, p.y, g.width, g.height, g.color
      FROM entity e
      JOIN graphics g ON g.id = e.id
      JOIN position p ON p.id = e.id
      ",
    )?;

    let bg_color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    self.target.clear(bg_color);

    while let State::Row = sql.next()? {
      self.render_entity(RenderData {
        x: sql.read::<f64>(0)?,
        y: sql.read::<f64>(1)?,
        width: sql.read::<f64>(2)?,
        height: sql.read::<f64>(3)?,
        color: sql.read::<String>(4)?,
      });
    }
    let font = SystemSource::new()
      .select_best_match(&[FamilyName::SansSerif], &Properties::new())
      .unwrap()
      .load()
      .unwrap();
    self.target.draw_text(
      &font,
      14.,
      &format!("fps: {:.1}", self.fps),
      Point::new(0., 100.),
      &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
      &DrawOptions::new(),
    );
    let (w, h) = self.window.get_size();
    self
      .window
      .update_with_buffer(self.target.get_data(), w, h)?;

    Ok(())
  }
  fn render_entity(&mut self, data: RenderData) {
    let rect_color = match data.color.as_ref() {
      "red" => color(0xff, 0, 0, 0xff),
      "green" => color(0, 0xff, 0, 0xff),
      "blue" => color(0, 0, 0xff, 0xff),
      _ => color(0x88, 0x88, 0x88, 0xff),
    };
    let mut pb = PathBuilder::new();
    pb.rect(
      (data.x - (data.width / 2.)) as f32,
      (data.y - (data.height / 2.)) as f32,
      data.width as f32,
      data.height as f32,
    );
    let path = pb.finish();
    self
      .target
      .fill(&path, &Source::Solid(rect_color), &DrawOptions::new());
  }
}

use crate::components::{Builder, Component};

use anyhow::anyhow;
use anyhow::Result;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
use sqlite::{Connection, State};

#[derive(Debug)]
struct RenderCommand {
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
  pub fn new(window: Window) -> Result<Self> {
    let (width, height) = window.get_size();
    let target = DrawTarget::new(width as i32, height as i32);

    Ok(App {
      fps: 0.,
      window,
      target,
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
      Self::render_entity(
        &mut self.target,
        RenderCommand {
          x: sql.read::<f64>(0)?,
          y: sql.read::<f64>(1)?,
          width: sql.read::<f64>(2)?,
          height: sql.read::<f64>(3)?,
          color: sql.read::<String>(4)?,
        },
      );
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
  fn render_entity(target: &mut DrawTarget, data: RenderCommand) {
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
    target.fill(&path, &Source::Solid(rect_color), &DrawOptions::new());
  }
}

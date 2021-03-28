use crate::app::App;
use anyhow::Result;
use minifb::Window;
use raqote::DrawOptions;
use raqote::DrawTarget;
use raqote::PathBuilder;
use raqote::SolidSource;
use raqote::Source;
use sqlite::{State, Statement};

pub struct RenderSystem<'a> {
    window: &'a mut Window,
    target: DrawTarget,
    sql: Statement<'a>,
}

#[derive(Debug)]
struct RenderData {
    shape: String,
    color: String,
    x: f64,
    y: f64,
}

impl<'a> RenderSystem<'a> {
    pub fn new(app: &'a mut App) -> Result<Box<RenderSystem>> {
        let (w, h) = app.window.get_size();
        Ok(Box::new(RenderSystem {
            window: &mut app.window,
            target: DrawTarget::new(w as i32, h as i32),
            sql: app.db.prepare(
                "
                SELECT g.shape, g.color, p.x, p.y
                FROM entity e
                JOIN graphics g ON g.id = e.id
                JOIN position p ON p.id = e.id
            ",
            )?,
        }))
    }
    pub fn render(&mut self) -> Result<()> {
        self.sql.reset()?;

        while let State::Row = self.sql.next()? {
            self.render_entity(RenderData {
                shape: self.sql.read::<String>(0)?,
                color: self.sql.read::<String>(1)?,
                x: self.sql.read::<f64>(2)?,
                y: self.sql.read::<f64>(3)?,
            });
        }
        let (w, h) = self.window.get_size();
        self.window
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

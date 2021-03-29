use crate::app::App;
use crate::system::System;
use anyhow::Result;
use sqlite::{Connection, State, Statement};

struct Pos {
  x: f64,
  y: f64,
}

#[derive(Debug)]
struct Block {
  x: f64,
  y: f64,
  width: f64,
  height: f64,
}

impl Block {
  pub fn overlaps(&self, other: &Block) -> bool {
    ((self.left() < other.right() && self.left() >= other.left())
      || (self.right() < other.right() && self.right() >= other.left()))
      && ((self.top() < other.bottom() && self.top() >= other.top())
        || (self.bottom() < other.bottom() && self.bottom() >= other.top()))
  }
  pub fn left(&self) -> f64 {
    self.x - (self.width / 2.)
  }
  pub fn right(&self) -> f64 {
    self.x + (self.width / 2.)
  }
  pub fn top(&self) -> f64 {
    self.y - (self.height / 2.)
  }
  pub fn bottom(&self) -> f64 {
    self.y + (self.height / 2.)
  }
}

#[cfg(test)]
mod tests {
  use super::Block;
  #[test]
  fn block_borders() {
    let a = Block {
      x: 10.,
      y: 10.,
      width: 20.,
      height: 20.,
    };
    assert_eq!(a.top(), 0.);
    assert_eq!(a.left(), 0.);
    assert_eq!(a.bottom(), 20.);
    assert_eq!(a.right(), 20.);
    let a = Block {
      x: 100.,
      y: 100.,
      width: 20.,
      height: 20.,
    };
    assert_eq!(a.top(), 90.);
    assert_eq!(a.left(), 90.);
    assert_eq!(a.bottom(), 110.);
    assert_eq!(a.right(), 110.);
  }
  #[test]
  fn overlap_blocks() {
    let a = Block {
      x: 10.,
      y: 10.,
      width: 20.,
      height: 20.,
    };
    let b = Block {
      x: 20.,
      y: 10.,
      width: 20.,
      height: 20.,
    };
    let c = Block {
      x: 40.,
      y: 10.,
      width: 20.,
      height: 20.,
    };
    assert!(a.overlaps(&b), "A overlaps B ...");
    assert!(b.overlaps(&c), "and B overlaps C ...");
    assert!(!a.overlaps(&c), "but A does NOT overlap C");
  }
}

pub struct CollisionSystem<'a> {
  sql: Statement<'a>,
}
impl<'a> System<'a> for CollisionSystem<'a> {
  fn new(connection: &'a Connection) -> Result<Box<Self>> {
    let statement = connection.prepare(
      "
      SELECT x, y, width, height
      FROM entity e
      JOIN position p ON p.id = e.id
      JOIN graphics g ON g.id = e.id
      ",
    )?;
    Ok(Box::new(CollisionSystem { sql: statement }))
  }
  fn tick(&mut self, _delta: f64) -> Result<()> {
    self.sql.reset()?;

    let mut blocks = Vec::new();
    while let State::Row = self.sql.next()? {
      blocks.push(Block {
        x: self.sql.read::<f64>(0)?,
        y: self.sql.read::<f64>(1)?,
        width: self.sql.read::<f64>(2)?,
        height: self.sql.read::<f64>(3)?,
      });
    }

    let mut collisions = Vec::new();
    for (i, a) in blocks.iter().enumerate() {
      for b in &blocks[(i + 1)..] {
        if a.overlaps(b) {
          println!("{:?} overlaps {:?}", a, b);
          assert!(false);
          collisions.push((i, a, b))
        }
      }
    }
    if collisions.len() > 0 {
      println!("collisions: {:?}", collisions.len());
    }

    Ok(())
  }
}

use crate::components::{Builder, Component};
use anyhow::Result;
use sqlite::Connection;

pub struct Database;

impl Database {
  pub fn initialize_tables(connection: &Connection) -> Result<()> {
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

    Ok(())
  }
}

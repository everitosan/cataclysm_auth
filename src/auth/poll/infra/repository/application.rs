use async_trait::async_trait;

use crate::infra::postgres::PostgresDB;
use crate::errors::Result;

use super::super::super::{
  dtos::application::CreateDto,
  domain::application::{ApplicationRepo, Application}
};

pub struct PsqlRepo<'a> {
  pub db: &'a PostgresDB
}

#[async_trait]
impl <'a> ApplicationRepo for PsqlRepo<'a> {

  async fn list(&self) -> Result<Vec<Application>> {
    let conn = self.db.pool.get().await?;
    let rows = conn.query("SELECT id, name, active, deleted, created FROM app WHERE deleted = false", &[]).await?;
    let apps = rows.into_iter().map(|row| {
      Application {
        id: row.get("id"),
        name: row.get("name"),
        active: row.get("active"),
        created: row.get("created"),
      }
    }).collect::<Vec<Application>>();

    Ok(apps)
  }

  async fn save(&self, dto: CreateDto) -> Result<Application> {
    let conn = self.db.pool.get().await?;
    let row = conn.query_one("INSERT INTO app (name) VALUES ($1) RETURNING *", &[&dto.name]).await?;
    Ok(Application{
      id: row.get("id"),
      name: row.get("name"),
      active: row.get("active"),
      created: row.get("created"),
    })
  }

  async fn find_by_id(&self, id: i32) -> Result<Option<Application>> {
    let conn = self.db.pool.get().await?;
    let rows = conn.query_opt("SELECT id, name, active, deleted, created FROM app WHERE (id = $1) AND (deleted = false) ", &[&id]).await?;

    match rows {
      Some(row) => {
        let application = Application {
          id: row.get("id"),
          name: row.get("name"),
          active: row.get("active"),
          created: row.get("created"),
        };
        Ok(Some(application))
    },
      None => {
        return Ok(None)
      }
    }

  }

  async fn delete(&self, id: i32) -> Result<()> {
    let conn = self.db.pool.get().await?;
    conn.execute("UPDATE app SET deleted = true WHERE id = $1", &[&id]).await?;
    Ok(())
  }

}
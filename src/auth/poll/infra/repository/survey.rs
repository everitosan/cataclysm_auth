use async_trait::async_trait;

use crate::errors::Result;
use crate::infra::postgres::PostgresDB;
use crate::modules::poll::domain::survey::Question;
use super::super::super::domain::survey::{Survey, SurveyRepo};
use super::super::super::dtos::survey::CreateDto;

#[derive(Clone)]
pub struct PsqlRepo<'a> {
  pub db: &'a PostgresDB
}

#[async_trait]
impl <'a> SurveyRepo for PsqlRepo<'a>  {
  
  async fn list(&self) -> Result<Vec<Survey>> {
    let conn = self.db.pool.get().await?;
    let rows = conn.query("SELECT s.id, s.app_id, s.name, s.created, s.active, s.questions FROM survey s RIGHT JOIN app a ON s.app_id = a.id WHERE (s.deleted = false) AND (a.deleted = false) ORDER BY created", &[]).await?;
    Ok(rows.into_iter().map(|row|{
      let v: serde_json::Value = row.get("questions");
      let questions = serde_json::from_value::<Vec<Question>>(v).unwrap_or_default();
      
      Survey{
        id: row.get("id"),
        app_id: row.get("app_id"),
        name: row.get("name"),
        created: row.get("created"),
        active: row.get("active"),
        questions
      }
    }).collect::<Vec<Survey>>())
  }

  async fn get_by_id(&self, id: i32) -> Result<Option<Survey>> {
    let conn = self.db.pool.get().await?;
    let opt = conn.query_opt("SELECT s.id, s.app_id, s.name, s.created, s.active, s.questions FROM survey s RIGHT JOIN app a ON s.app_id = a.id WHERE (s.id = $1) AND (s.deleted = false) AND (a.deleted = false)", &[&id]).await?;
    match opt {
      Some(row) => {
        let v: serde_json::Value = row.get("questions");
        let questions = serde_json::from_value::<Vec<Question>>(v).unwrap_or_default();

        Ok(Some(Survey{
          id: row.get("id"),
          app_id: row.get("app_id"),
          name: row.get("name"),
          active: row.get("active"),
          questions,
          created: row.get("created")
        }))
      },
      None => {
        Ok(None)
      }
    }
  }

  async fn get_by_app_id(&self, id: i32) -> Result<Vec<Survey>> {
    let conn = self.db.pool.get().await?;
    let rows = conn.query("SELECT s.id, s.app_id, s.name, s.created, s.active, s.questions FROM survey s RIGHT JOIN app a ON s.app_id = a.id WHERE (s.app_id = $1) AND (s.deleted = false) AND (a.deleted = false)", &[&id]).await?;
    Ok(rows.into_iter().map(|row|{
      let v: serde_json::Value = row.get("questions");
      let questions = serde_json::from_value::<Vec<Question>>(v).unwrap_or_default();
      
      Survey{
        id: row.get("id"),
        app_id: row.get("app_id"),
        name: row.get("name"),
        created: row.get("created"),
        active: row.get("active"),
        questions
      }
    }).collect::<Vec<Survey>>())
  }

  async fn save(&self, dto: CreateDto) -> Result<Survey> {
    let conn = self.db.pool.get().await?;
    let row = conn.query_one("INSERT INTO survey (app_id, name) VALUES ($1, $2) RETURNING *", &[&dto.app_id, &dto.name]).await?;

    Ok(Survey{
      id: row.get("id"),
      app_id: row.get("app_id"),
      name: row.get("name"),
      active: row.get("active"),
      questions: vec![],
      created: row.get("created")
    })
  }

  async fn add_questions(&self, id: i32, questions: Vec<Question>) -> Result<Survey> {
    let conn = self.db.pool.get().await?;
    let q_str = serde_json::to_string(&questions).unwrap();
    let query = format!("UPDATE survey SET questions = questions || '{}' WHERE id = {} RETURNING *", &q_str, &id);
    let row = conn.query_one(&query, &[]).await?;
    let v: serde_json::Value = row.get("questions");
    let questions = serde_json::from_value::<Vec<Question>>(v).unwrap_or_default();
    Ok(Survey{
      id: row.get("id"),
      app_id: row.get("app_id"),
      name: row.get("name"),
      active: row.get("active"),
      questions,
      created: row.get("created")
    })
  }

  async fn delete(&self, id: i32) -> Result<()> {
    let conn = self.db.pool.get().await?;
    conn.execute("UPDATE survey SET deleted = true WHERE id = $1", &[&id]).await?;
    Ok(())
  }
}
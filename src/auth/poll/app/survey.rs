use chrono::Utc;
use uuid::Uuid;
use validator::Validate;
use crate::{errors::Result, state::Repositories};


use super::super::{
  domain::survey::{Survey, Question},
  dtos::{survey, question}
};

use crate::utils::parse_id;

pub async fn get_all(repos: &Repositories) -> Result<Vec<Survey>> {
  repos.survey.list().await
}


pub async fn create(repos: &Repositories, dto: survey::CreateDto) -> Result<Survey> {
  dto.validate()?;
  repos.survey.save(dto).await
}


pub async fn get_by_app_id(repos: &Repositories, id_str: String) -> Result<Option<Vec<Survey>>> {
  let app_id = parse_id(id_str)?;
  if let Some(app) = repos.application.find_by_id(app_id).await? {
    let surveys = repos.survey.get_by_app_id(app.id).await?;
    return Ok(Some(surveys));
  }
  Ok(None)
}


pub async fn get_by_id(repos: &Repositories, id_str: String) -> Result<Option<Survey>> {
  let id = parse_id(id_str)?;
  repos.survey.get_by_id(id).await
}


pub async fn delete(repos: &Repositories, id_str: String) -> Result<()> {
  let id = parse_id(id_str)?;
  repos.survey.delete(id).await
}


pub async fn add_questions(repos: &Repositories, id_str: String, questions: Vec<question::CreateDto>) -> Result<Option<Survey>> {
  let id = parse_id(id_str)?;
  
  let survey_opt = repos.survey.get_by_id(id).await?;

  if survey_opt.is_some() {  
    let qs: Vec<Question> = questions.into_iter().map(|q|{
      Question {
        id:  Uuid::new_v4().to_string(),
        text: q.text,
        q_type: q.q_type,
        order: q.order,
        required: q.required,
        active: true,
        deleted: false,
        created: Utc::now().naive_local()
      }
    }).collect();
    let res = repos.survey.add_questions(id, qs).await?;
    Ok(Some(res))
  } else {
    Ok(None)
  }
}



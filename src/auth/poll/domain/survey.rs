use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use super::super::dtos::survey::CreateDto;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum QuestionType {
  Open,
  YesNo,
  Range {
    limit: u8,
    labels: Vec<String>,
  },
  Option {
    options: Vec<String>
  },
  Multiple {
    options: Vec<String>
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
  pub id: String,
  pub text: String,
  pub q_type: QuestionType,
  pub order: u8,
  pub required: bool,
  pub active: bool,
  pub deleted: bool,
  pub created: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Survey {
  pub id: i32,
  pub app_id: i32,
  pub name: String,
  pub created: NaiveDateTime,
  pub active: bool,
  pub questions: Vec<Question>
} 
#[async_trait]
pub trait SurveyRepo {
  async fn list(&self) -> Result<Vec<Survey>>;
  async fn get_by_app_id(&self, id: i32) -> Result<Vec<Survey>>;
  async fn save(&self, dto: CreateDto) -> Result<Survey>;
  async fn get_by_id(&self, id: i32) -> Result<Option<Survey>>;
  async fn add_questions(&self, id: i32, questions: Vec<Question>) -> Result<Survey>;
  async fn delete(&self, id: i32) -> Result<()>;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn survey_structure() {
    let open_q = Question {
      id: "020239203".to_owned(),
      text: "Escribe tus comentarios adicionales".to_owned(),
      q_type: QuestionType::Open,
      order: 4,
      required: true,
      active: true,
      deleted: false,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    let range_q = Question {
      id: "02392309".to_owned(),
      text: "¿Qué tan satisfecho estás con la aplicación?".to_owned(),
      q_type: QuestionType::Range { 
        labels: vec!["Muy satisfecho".to_owned(), "Poco satisfecho".to_owned()],
        limit: 5,
      },
      required: true,
      active: true,
      deleted: false,
      order: 0,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    let yesno_q = Question {
      id: "923180321".to_owned(),
      text: "¿Recomendarías esta aplicación a alguien más?".to_owned(),
      q_type: QuestionType::YesNo,
      order: 1,
      required: true,
      active: true,
      deleted: false,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    let option_q = Question {
      id: "923180322".to_owned(),
      text: "¿Qué fue lo que más te gustó de la aplicación?".to_owned(),
      q_type: QuestionType::Option { 
        options: vec!["Facilidad".to_owned()], 
      },
      order: 2,
      required: true,
      active: true,
      deleted: false,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    let multiple_q = Question {
      id: "923180321".to_owned(),
      text: "¿Qué fue lo que menos te gustó de la aplicación?".to_owned(),
      q_type: QuestionType::Multiple {
        options: vec!["El diseño".to_owned()]
      },
      order: 3,
      required: true,
      active: true,
      deleted: false,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    let questions: Vec<Question> = vec![open_q, range_q, yesno_q, option_q, multiple_q];

    let survey = Survey {
      id: 10,
      app_id: 12,
      name: "Primera".to_owned(),
      active: false,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap(),
      questions
    };

    assert_eq!(survey.name, "Primera".to_owned());
    assert_eq!(survey.active, false);
    assert_eq!(survey.questions.len(), 5);

  }
}
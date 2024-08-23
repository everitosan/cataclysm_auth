use validator::Validate;
use serde::{Deserialize, Serialize};
use crate::{errors::Result, modules::poll::domain::survey::QuestionType};

/*
* Create Question Dto
*/
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateDto {
  #[validate(length(min = 5))]
  pub text: String,
  pub q_type: QuestionType,
  #[validate(range(min = 0))]
  pub order: u8,
  pub required: bool,
}

impl CreateDto {
  pub fn new(text: String, q_type: QuestionType, order: u8, required: bool) -> Result<Self> {
    let dto = CreateDto{
      text, q_type, required, order
    };
    dto.validate()?;
    Ok(dto)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json::from_str;

  #[test]
  #[should_panic]
  fn create_question_dto_fails_by_name() {
    CreateDto::new("".to_owned(), QuestionType::Open, 0, true).unwrap();
  }

  #[test]
  #[should_panic]
  fn create_question_dto_fails_deserialize_by_order() {
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": "Open",
        "required": true,
        "order": -1
      }
    "#;

    from_str::<CreateDto>(&dto_string).unwrap();
  }

  #[test]
  fn create_question_dto_deserialize_open() { 
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": "Open",
        "required": true,
        "order": 0
      }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.order, 0);  
    assert_eq!(dto.q_type, QuestionType::Open);  
  }

  #[test]
  fn create_question_dto_deserialize_yesno() { 
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": "YesNo",
        "required": true,
        "order": 0
      }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.order, 0);  
    assert_eq!(dto.q_type, QuestionType::YesNo);  
  }

  #[test]
  fn create_question_dto_deserialize_range() { 
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": {
          "Range": {
            "limit": 10,
            "labels": ["uno", "dos"]
          }
        },
        "required": true,
        "order": 0
      }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.order, 0);  
    assert_eq!(dto.q_type, QuestionType::Range { limit: 10, labels: vec!["uno".to_owned(), "dos".to_owned()] });  
  }


  #[test]
  fn create_question_dto_deserialize_option() { 
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": {
          "Option": {
            "options": ["uno", "dos"]
          }
        },
        "required": true,
        "order": 0
      }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.order, 0);  
    assert_eq!(dto.q_type, QuestionType::Option { options: vec!["uno".to_owned(), "dos".to_owned()] });  
  }

  #[test]
  fn create_question_dto_deserialize_multiple() { 
    let dto_string = r#"
      {
        "text": "Question text?",
        "qType": {
          "Multiple": {
            "options": ["uno", "dos"]
          }
        },
        "required": true,
        "order": 0
      }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.order, 0);  
    assert_eq!(dto.q_type, QuestionType::Multiple { options: vec!["uno".to_owned(), "dos".to_owned()] });  
  }
}
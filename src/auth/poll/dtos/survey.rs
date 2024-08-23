use validator::Validate;
use serde::{Deserialize, Serialize};
use crate::errors::Result;

/*
* Create Survey Dto
*/
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateDto {
  pub app_id: i32,
  #[validate(length(min=5))]
  pub name: String,
}

impl CreateDto {
  pub fn new(app_id: i32, name: String) -> Result<Self> {
    let dto = CreateDto {
      app_id,
      name
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
  fn create_survey_dto_fails_by_app_id() {
    CreateDto::new(-1, "dasd".to_owned()).unwrap();
  }

  #[test]
  #[should_panic]
  fn create_survey_dto_fails_by_name() {
    CreateDto::new(09, "".to_owned()).unwrap();
  }

  #[test]
  fn create_survey_dto() {
    let app_id = 09;
    let survey_name = "final".to_owned();
    let survey = CreateDto::new(app_id.clone(), survey_name.clone()).unwrap();
    assert_eq!(survey.app_id, app_id);
    assert_eq!(survey.name, survey_name);
  }

  #[test]
  fn create_survey_dto_deserialize() {
    let dto_string = r#"
    {
      "name": "Sat ID",
      "appId": 909
    }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.name, "Sat ID".to_owned());
    assert_eq!(dto.app_id, 909);
  }
}
use validator::Validate;
use serde::{Deserialize, Serialize};
use crate::errors::Result;

/*
* Create Application Dto
*/
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateDto {
  #[validate(length(min = 5))]
  pub name: String
}

impl CreateDto {
  pub fn new(name: String) -> Result<Self> {
    let dto = CreateDto {
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
  fn create_app_dto_validation_throws_err() {
    CreateDto::new("".to_owned()).unwrap();
  }
  #[test]
  fn create_app_dto() {
    let app = CreateDto::new("Sat ID".to_owned()).unwrap();
    assert_eq!(app.name, "Sat ID".to_owned());
  }

  #[test]
  fn create_dto_deserialize() {
    let dto_string = r#"
    {
      "name": "Sat ID"
    }
    "#;
    let dto = from_str::<CreateDto>(&dto_string).unwrap();
    assert_eq!(dto.name, "Sat ID".to_owned());
  }

}
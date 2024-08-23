use validator::Validate;

use crate::{errors::Result, state::Repositories};
use super::super::{
  domain::application::Application,
  dtos::application::CreateDto
};
use crate::utils::parse_id;

pub async fn get_all(repos: &Repositories) -> Result<Vec<Application>> {
  repos.application.list().await
}

pub async fn find_by_id(repos: &Repositories, id_str: String) -> Result<Option<Application>> {
	let id = parse_id(id_str)?;
  repos.application.find_by_id(id).await
}

pub async fn create(repos: &Repositories, dto: CreateDto) -> Result<Application> {
  dto.validate()?;
  repos.application.save(dto).await
}

pub async fn delete(repos: &Repositories, id_str: String) -> Result<()> {
  let id = parse_id(id_str)?;
  repos.application.delete(id).await
}

use serde_json;
use log::{error, warn};
use crate::{errors::Error, state::State};
use cataclysm::{http::{Method, Path, Response}, Branch, Shared};
use super::super::super::dtos::survey::CreateDto as CreateSurveyDto;
use super::super::super::dtos::question::CreateDto as CreateQuestionDto;


use crate::modules::poll::app::survey;

async fn list_surveys(shared: Shared<State>) -> Response {
	
	match survey::get_all(&shared.repositories).await {
		Ok(surveys) => {
			let response = serde_json::to_string(&surveys).unwrap();
			Response::ok().header("Content-Type", "application/json").body(response)
		},
		Err(e) => {
			error!("{}", e);
			Response::internal_server_error()
		}
	}
}


async fn get_by_id(path: Path<(String,)>, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();
	match survey::get_by_id(&shared.repositories, id_str).await {
		Ok(survey_opt) => {
			if let Some(survey) = survey_opt {
				let response = serde_json::to_string(&survey).unwrap();
				return Response::ok().header("Content-Type", "application/json").body(response)
			}
			Response::not_found()
		},
		Err(e) => {
			match e {
				Error::Input(e_str) => {
					warn!("{}", e_str);
					Response::bad_request().body(e_str)
				},
				_ => {
					error!("{}", e);
					Response::internal_server_error()
				}
			}
		}
	}
}

async fn create_survey(body: String, shared: Shared<State>) -> Response {
	let body: CreateSurveyDto = match serde_json::from_str::<CreateSurveyDto>(&body) {
		Ok(b) => {b},
		Err(e) => {
			warn!("{}",e);
			return Response::bad_request()
		},
	};

	match survey::create(&shared.repositories, body).await {
		Ok(survey) => {
			let response = serde_json::to_string(&survey).unwrap();
			Response::created().header("Content-Type", "application/json").body(response)
		},
		Err(e) => {
			match e {
				Error::Validation(e_str) => {
					warn!("{}", e_str);
					Response::bad_request()
				},
				_ => {
					error!("{}", e);
					Response::internal_server_error()
				}
			}
		
		}
	}
}

async fn add_questions(path: Path<(String,)>, body: String, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();
	let body: Vec<CreateQuestionDto> = match serde_json::from_str::<Vec<CreateQuestionDto>>(&body) {
		Ok(b) => {b},
		Err(e) => {
			warn!("{}",e);
			return Response::bad_request()
		},
	};
	match survey::add_questions(&shared.repositories, id_str, body).await {
		Ok(survey_opt) => {
			if let Some(survey) = survey_opt {
				let response = serde_json::to_string(&survey).unwrap();
				return Response::ok().header("Content-Type", "application/json").body(response)
			}
			Response::not_found()
		},
		Err(e) => {
			match e {
				Error::Input(e_str) => {
					warn!("{}", e_str);
					Response::bad_request()
				},
				Error::Validation(e_str) => {
					warn!("{}", e_str);
					Response::bad_request()
				},
				_ => {
					error!("{}", e);
					Response::bad_request()
				}
			}
		}
	}
}

async fn delete(path: Path<(String,)>, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();
	
	match survey::delete(&shared.repositories, id_str).await {
		Ok(()) => {
			Response::ok()
		},
		Err(e) => {
			match e {
				Error::Input(e_str) => {
					warn!("{}", e_str);
					Response::bad_request()
				},
				_ => {
					Response::internal_server_error()					
				}
			}
		}
	}
}

pub fn get_endpoints() -> Branch<State> {
	Branch::new("/survey")
		.with(Method::Get.to(list_surveys))
		.with(Method::Post.to(create_survey))
		.nest(
			Branch::new("{:id}")
				.with(Method::Get.to(get_by_id))
				.with(Method::Delete.to(delete))
				.nest(
					Branch::new("/question")
						.with(Method::Put.to(add_questions))
				)
		)
}

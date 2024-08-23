use log::{error, warn};
use serde_json;
use cataclysm::{http::{Method, Response, Request, Path}, Branch, Shared};
use cataclysm_auth::auth;

use crate::state::State;
use crate::errors::Error;

use crate::modules::poll::{
	dtos::application::CreateDto,
	app::{application, survey}
};


// Retrieve all applications
#[auth(prefix="Bearer")]
async fn list_applications(shared: Shared<State>, req: Request) -> Response {
	let applications = match application::get_all(&shared.repositories).await {
		Ok(a) => {a},
		Err(e) => {
			error!("{}", e);
			return Response::internal_server_error();
		}
	};

	let response = serde_json::to_string(&applications).unwrap();
	Response::ok().header("Content-Type", "application/json").body(response)
}

// Retrieve an application by it's id
async fn get_by_id(path: Path<(String,)>, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();
	match application::find_by_id(&shared.repositories, id_str).await {
		Ok(opt) => {
			match opt {
				Some(application) => {
					let response = serde_json::to_string(&application).unwrap();
					Response::ok().header("Content-Type", "application/json").body(response)
				}, 
				None => { Response::not_found() }
			}
		},
		Err(e) => {
			error!("{}", e);
			match e {
				Error::Input(e_str) => {
					Response::bad_request().body(e_str)
				},
				_ => {
					Response::internal_server_error()
				}
			}
		}
	}
}

// Register a new application
async fn create_application(body: String, shared: Shared<State>) -> Response {

	let body: CreateDto = match serde_json::from_str::<CreateDto>(&body) {
		Ok(b) => {b},
		Err(e) => {
			error!("{}", e);
			return Response::bad_request()
		},
	};

	let application = match application::create(&shared.repositories, body).await {
		Ok(a) => {a},
		Err(e) => {
			match e {
				Error::Validation(e_str) => {
					warn!("{}", e_str);
					return Response::bad_request()
				},
				_ => {
					error!("{}", e);
					return Response::internal_server_error()
				}
			}
		}
	};

	let response = serde_json::to_string(&application).unwrap();
	Response::created().header("Content-Type", "application/json").body(response)
}


async fn delete(path: Path<(String,)>, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();

	match application::delete(&shared.repositories, id_str).await {
		Ok(()) => {
			Response::ok()
		},
		Err(e) => {
			error!("{}", e);
			Response::internal_server_error()
		}
	}
}


// Retrieve surveys related to an application
async fn list_application_surveys(path: Path<(String,)>, shared: Shared<State>) -> Response {
	let (id_str,) = path.into_inner();

	match survey::get_by_app_id(&shared.repositories, id_str).await {
		Ok(opt) => {
			if let Some(surveys) = opt {
				let response = serde_json::to_string(&surveys).unwrap();
				return Response::ok().header("Content-Type", "application/json").body(response);
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
					warn!("{}", e);
					Response::internal_server_error()
				}
			}
		}
	}
}


pub fn get_endpoints() -> Branch<State> {
	Branch::new("/application")
		.with(Method::Get.to(list_applications))
		.with(Method::Post.to(create_application))
		.nest(
			Branch::new("{:id}")
			.with(Method::Get.to(get_by_id))
			.with(Method::Delete.to(delete))
			.nest(
				Branch::new("/surveys")
					.with(Method::Get.to(list_application_surveys)))
	)
}

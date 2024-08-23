use clap::{Subcommand, Args};

use crate::state::Repositories;
use crate::modules::poll::app::application;

#[derive(Debug, Args)]
pub struct ApplicationCommand {
  #[clap(subcommand)]
  pub command: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
  /// List all applications
  List
}


pub async fn execute(action: Action, repos: &Repositories) {
  match action {
    Action::List => {
      match application::get_all(repos).await {
        Ok(apps) => {
          println!("{:?}", apps);
        },
        Err(e) => {
          panic!("{}", e);
        }
      }
    }
  }
}
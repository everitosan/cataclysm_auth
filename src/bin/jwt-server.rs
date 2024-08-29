use colored::*;
use cataclysm::{http::{Method, Request, Response}, Branch, Server};
use cataclysm_auth::{jwt_protect, auth::jwt::{create, TokenType}};


async fn index() -> Response {
  let message = format!("Hello world!");
  Response::ok().body(message)
}

#[jwt_protect(prefix="Bearer", roles="admin")]
async fn only_for_admin(req: Request) -> Response {
  let message = format!("Hello admin {}!", claim.sub);
  Response::ok().body(message)
}
 
#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();

  let addr = "localhost:8000";

  let server = Server::builder(
    Branch::<()>::new("/").with(Method::Get.to(index))
      .nest(
        Branch::<()>::new("/admin").with(Method::Get.to(only_for_admin))
      )
  ).build().unwrap();

  if let Some(tkn) = create(
      TokenType::Access, 
      "SAHE922A".to_owned(),
      vec!["admin".to_owned(), "visor".to_owned()]
    ) {

    println!("🤖 Available endpoints:");
    println!("\t{} (curl http://{})", "- Home".green(), addr);
    println!("\t{} (curl http://{}/admin -H \"Authorization: Bearer {}\" -v)", "- Admin".green(), addr, tkn);
    
    server.run(addr).await.unwrap();
  }
 
}
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

#[jwt_protect(prefix="Bearer", roles="super")]
async fn only_for_super(req: Request) -> Response {
  let message = format!("Hello super {}!", claim.sub);
  Response::ok().body(message)
}

#[jwt_protect(prefix="Bearer", roles="visor")]
async fn only_for_visor(req: Request) -> Response {
  let message = format!("Hello visor {}!", claim.sub);
  Response::ok().body(message)
}
 
#[tokio::main]
async fn main() {
  dotenvy::from_filename("jwt.env").unwrap();

  let addr = "localhost:8000";

  let server = Server::builder(
    Branch::<()>::new("/").with(Method::Get.to(index))
      .nest(Branch::<()>::new("/admin").with(Method::Get.to(only_for_admin)))
      .nest(Branch::<()>::new("/super").with(Method::Get.to(only_for_super)))
      .nest(Branch::<()>::new("/visor").with(Method::Get.to(only_for_visor)))
  ).build().unwrap();

  if let Ok(tkn) = create(
      TokenType::Access, 
      "SAHE922A".to_owned(),
      vec!["admin".to_owned(), "visor".to_owned()]
    ) {

    // println!("🤖 Available endpoints:");
    // println!("\t{} (curl http://{})", "- Home".green(), addr);
    // println!("\t{} (curl http://{}/admin -H \"Authorization: Bearer {}\" -v)", "- Admin".green(), addr, tkn);

    let open_resource = format!("curl http://{}", addr);
    let admin_resource = format!("curl http://{}/admin -H \"Authorization: Bearer $JWT_TKN\"", addr);
    let super_resource = format!("curl http://{}/super -H \"Authorization: Bearer $JWT_TKN\" -v", addr);
    let visor_resource = format!("curl http://{}/visor -H \"Authorization: Bearer $JWT_TKN\"", addr);

    println!("🤖 JWT server:");
    println!("\t1.- Set env var with admin and visor roles");
    println!("\t\t {}", format!("JWT_TKN=\"{}\"", tkn).green());
    println!("\t2.- Request an open resource:");
    println!("\t\t {}", open_resource.blue());
    println!("\t3.- Request an admin resource:");
    println!("\t\t {}", admin_resource.blue());
    println!("\t4.- Request a super resource:");
    println!("\t\t {}", super_resource.blue());
    println!("\t5.- Request a visor resource:");
    println!("\t\t {}", visor_resource.blue());
    
    server.run(addr).await.unwrap();
  }
 
}
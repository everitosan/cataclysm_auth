use colored::*;
use cataclysm::{http::{Method, Response}, session::{CookieSession, Session}, Branch, Server};
use cataclysm_auth::cookie_protect;

async fn index(mut session: Session) -> Response {
  session.set("roles", "visor,admin");
  session.apply(
    Response::ok().body("Cookie setted!")
  )
}

#[cookie_protect(key="roles", roles="admin")]
async fn only_for_admin(session: Session) -> Response {
  let message = format!("Hello super admin!");
  Response::ok().body(message)
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().unwrap();

  let addr = "localhost:8000";

  let server = Server::builder(
    Branch::<()>::new("/").with(Method::Get.to(index))
      .nest(Branch::<()>::new("/admin").with(Method::Get.to(only_for_admin)))
  )
  .session_creator(CookieSession::new())
  .build().unwrap();

  let url = format!("http://{}", addr);
  let protected_url = format!("http://{}/admin", addr);
  
  println!("🍪 Cookie server:");
  println!("1.- Visit in a browser receive forbidden response {}", protected_url.green());
  println!("2.- Visit in a browser to get a valid cookie {}", url.green());
  println!("3.- Visit in a browser to see role-protected message {}", protected_url.green());
  server.run(addr).await.unwrap();
}
use colored::*;
use cataclysm::{http::{Method, Response}, session::Session, Branch, Server};
use cataclysm_auth::{cookie_protect, auth::cookie::CookieSession};

async fn index(mut session: Session) -> Response {
  session.set("roles", "visor,admin");
  session.apply(
    Response::ok().body("Cookie setted for admin and visor roles!")
  )
}

#[cookie_protect(key="roles", roles="admin")]
async fn only_for_admin(session: Session) -> Response {
  let message = format!("Hello admin!");
  Response::ok().body(message)
}

#[cookie_protect(key="roles", roles="visor")]
async fn only_for_visor(session: Session) -> Response {
  let message = format!("Hello visor!");
  Response::ok().body(message)
}

#[cookie_protect(key="roles", roles="user")]
async fn only_for_users(session: Session) -> Response {
  let message = format!("Hello user!");
  Response::ok().body(message)
}


#[tokio::main]
async fn main() {
  dotenvy::from_filename("cookie.env").unwrap();

  let addr = "localhost:8000";

  let server = Server::builder(
    Branch::<()>::new("/").with(Method::Get.to(index))
      .nest(Branch::<()>::new("/admin").with(Method::Get.to(only_for_admin)))
      .nest(Branch::<()>::new("/user").with(Method::Get.to(only_for_users)))
      .nest(Branch::<()>::new("/visor").with(Method::Get.to(only_for_visor)))
  )
  .session_creator(CookieSession::from_env())
  .build().unwrap();

  let url = format!("http://{}", addr);
  let admin_protected_url = format!("http://{}/admin", addr);
  let user_protected_url = format!("http://{}/user", addr);
  let visor_protected_url = format!("http://{}/visor", addr);
  
  println!("🍪 Cookie server:");
  println!("\t1.- Open in a browser receive forbidden response {}", admin_protected_url.green());
  println!("\t2.- Open in a browser to get a valid cookie for admin and visor {}", url.green());
  println!("\t3.- Open in a browser to see admin protected message {}", admin_protected_url.green());
  println!("\t4.- Open in a browser to see visor protected message {}", visor_protected_url.green());
  println!("\t5.- Open in a browser to see role-protected error{}", user_protected_url.green());
  server.run(addr).await.unwrap();
}
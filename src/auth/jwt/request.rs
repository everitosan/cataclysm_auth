use cataclysm::http::Request;

pub fn extact_jwt(req: Request, prefix: &str) -> Option<String> {
  let all_authorizations = req.headers.get("Authorization")?;

  let required_header = all_authorizations
    .into_iter()
    .filter(|h| h.contains(prefix))
    .collect::<Vec<&std::string::String>>();

  if required_header.len() == 0 {
    return None
  }

  let splited: Vec<&str> = required_header[0].split(prefix).collect();


  let tkn = splited.get(1)?;
  let response = format!("{}", tkn);
  Some(response)
}

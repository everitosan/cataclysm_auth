use chrono::{DateTime, Local};

pub trait FeedBackAnswer {}

pub struct Answer {
  pub question_id: String,
  pub answer: String
}

impl FeedBackAnswer for Answer {}

pub struct MultipleAnswer {
  pub question_id: String,
  pub answer: Vec<String>
}

impl FeedBackAnswer for MultipleAnswer {}

 pub struct Feedback<'a> {
  pub id: String,
  pub survey_id: String,
  pub folio: String,
  pub uuid: String,
  pub pending: bool,
  pub answers: Vec<&'a dyn FeedBackAnswer>,
  pub created:  DateTime<Local>
 } 

 #[cfg(test)]
 mod test {
  use super::*;

  #[test]
  fn feedback_structure() {
    let a1 = Answer {
      question_id: "12123".to_owned(),
      answer: "No hay nda malo".to_owned()
    };

    let a2 = MultipleAnswer {
      question_id: "12123".to_owned(),
      answer: vec!["No hay nda malo".to_owned()]
    };

    let answers: Vec<&dyn FeedBackAnswer> = vec![&a1, &a2];

    let f1 = Feedback {
      id: "2323232".to_owned(),
      survey_id: "23123".to_owned(),
      folio: "123123".to_owned(),
      uuid: "213213123".to_owned(),
      pending: false,
      answers,
      created: Local::now() 
    };

    assert_eq!(f1.answers.len(), 2);
  }
 }

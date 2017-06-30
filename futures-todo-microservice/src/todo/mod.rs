use std::io::{Error as IoError, ErrorKind};
use std::string::ToString;

use futures::{Async, Poll};
use futures::future::Future;

pub mod api;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    title: String,
    desc: String,
    status: TodoStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoStatus {
    Todo,
    InProgress,
    NeedsFeedback,
    Done
}

enum TodoRepository {
    Error,
    Create(Todo),
    Read(usize),
    Update(Todo),
    Delete(usize)
}

impl Future for TodoRepository {
    type Item = Option<Todo>;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            &mut TodoRepository::Create(_) => {
                // TODO INSERT INTO DB
                Err(IoError::new(ErrorKind::Other, "Not implemented"))
            }
            &mut TodoRepository::Read(id) => {
                // TODO SELECT FROM DB
                Ok(Async::Ready(Some(Todo{
                    id: id,
                    title: "Do your future homework".to_string(),
                    desc: "Do your future homework using futures".to_string(),
                    status: TodoStatus::InProgress
                })))
            },
            &mut TodoRepository::Update(_) => {
                // TODO UPDATE INTO DB

                Err(IoError::new(ErrorKind::Other, "Not implemented"))
            },
            &mut TodoRepository::Delete(_) => {
                // TODO DELETE FROM DB
                Err(IoError::new(ErrorKind::Other, "Not implemented"))
            }
            &mut TodoRepository::Error => {
                Err(IoError::new(ErrorKind::Other, "Not implemented"))
            }
        }
    }
}


impl ToString for Todo {
    fn to_string(&self) -> String {
        format!("{} {}: {} - {}", self.status.to_string(), self.id, self.title, self.desc)
    }
}

impl ToString for TodoStatus {
    fn to_string(&self) -> String {
        match *self {
            TodoStatus::Todo => "TODO".to_string(),
            TodoStatus::InProgress => "INPROGRESS".to_string(),
            TodoStatus::NeedsFeedback => "FEEDBACK".to_string(),
            TodoStatus::Done => "DONE".to_string(),
        }
    }
}

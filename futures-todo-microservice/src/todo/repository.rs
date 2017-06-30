use std::io::{Error as IoError, ErrorKind};
use std::string::ToString;

use futures::{Async, Poll};
use futures::future::Future;

use super::entity::{Todo, TodoStatus};

pub enum TodoRepository {
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
                Ok(Async::Ready(Some(Todo::new(
                    id,
                    "Do your future homework".to_string(),
                    "Do your future homework using futures".to_string(),
                    TodoStatus::InProgress
                ))))
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

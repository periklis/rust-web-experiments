use std::convert::Into;
use std::string::ToString;

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


impl Todo {
    fn get(id: usize) -> Option<Todo> {
        Some(Todo{
            id: id,
            title: "Do your future homework".to_string(),
            desc: "Do your future homework using futures".to_string(),
            status: TodoStatus::InProgress
        })
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

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
    pub fn new(id: usize, title: String, desc: String, status: TodoStatus) -> Self {
        Todo {id: id, title: title, desc: desc, status: status}
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

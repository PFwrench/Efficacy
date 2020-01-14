use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub enum TaskState {
    Todo,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub description: String,
    pub state: TaskState,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    pub context_name: String
}

use serder::Deserialize;
use serde::Serialize;
use uiid::Uuid;
use strum_macros::{EnumString, Display};

pub enum TaskState{
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Paused,
}


#[derive(Serialize)]
pub struct Task{
    pub user_uiid: String,
    pub task_uiid: String,
    pub task_type: String,
    pub state : String,
    pub soure_file :String,
    pub result_file :Option<String>,


}
impl Task{
    pub fn new(user_uiid: String, task_type: String, source_file: String) -> Self {
        Task {
            user_uiid,
            task_uiid: Uuid::new_v4().to_string(),
            task_type,
            state: TaskState::NotStarted.to_string(),
            source_file,
            result_file: None,
        }
    }
}
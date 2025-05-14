use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use strum_macros::{EnumString, Display};

#[derive(Serialize,EnumString,Display,Eq,PartialEq,Debug)]
pub enum TaskState{
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Paused,
}


#[derive(Serialize,Debug)]
pub struct Task{
    pub user_uiid: String,
    pub task_uiid: String,
    pub task_type: String,
    pub state : TaskState,
    pub source_file :String,
    pub result_file :Option<String>,


}
impl Task{
    pub fn new(user_uiid: String, task_type: String, source_file: String) -> Task {
        Task {
            user_uiid,
            task_uiid: Uuid::new_v4().to_string(),
            task_type,
            state:  TaskState::NotStarted,
            source_file,
            result_file: None,
        }
    }
    pub fn get_global_id(&self)-> String {
        format!("{}#{}", self.user_uiid, self.task_uiid)
    }

    pub fn can_transition_to(&self,state:&TaskState)-> bool{
        self.state != *state
    }
}
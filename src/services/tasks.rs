use crate::domain::models::tasks::Task;
use crate::domain::commands::tasks::CreateTaskCommand;
use crate::services::handlers::commands::tasks::create_task_handler;
use crate::services::handlers::queries::tasks::list_tasks_handler;
use chrono::NaiveDate;

pub struct TaskService {
}


impl TaskService {

    pub fn new() -> Self {
        TaskService {}
    }


    pub fn create(&self, title: String, description: String, assigner: String, due_date: NaiveDate) -> Task {
        // receives args from the caller, constructs a command, and calls the handler
        let command = CreateTaskCommand {
            title,
            description,
            assigner,
            due_date,
        };

        let task = create_task_handler(command);
        task

    }

    pub fn list(&self) -> Vec<Task> {
        // we only have a single handler for listing tasks
        let tasks = list_tasks_handler();
        tasks
    }
}
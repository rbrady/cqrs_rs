use crate::domain::models::tasks::Task;
use crate::domain::commands::tasks::CreateTaskCommand;
use crate::services::handlers::commands::tasks::create_task_handler;
use crate::services::handlers::queries::tasks::list_tasks_handler;
use chrono::NaiveDate;

pub struct TaskService<'a> {
    tasks: &'a mut Vec<Task>, // Store tasks as state
}


impl<'a> TaskService<'a> {

    pub fn new(tasks: &'a mut Vec<Task>) -> Self {
        TaskService { tasks }
    }


    pub fn create(&mut self, title: String, description: String, assigner: String, due_date: NaiveDate) -> Task {
        // receives args from the caller, constructs a command, and calls the handler
        let command = CreateTaskCommand {
            title,
            description,
            assigner,
            due_date,
        };

        create_task_handler(command, &mut self.tasks)
    }

    pub fn list(&mut self) -> Vec<Task> {
        // we only have a single handler for listing tasks
        list_tasks_handler(&mut self.tasks)
    }

}
use crate::adapters::tasks::TaskRepository;
use crate::domain::models::tasks::Task;

// create a list task handler
pub fn list_tasks_handler() -> Vec<Task> {
    let repository = TaskRepository::new();
    repository.list()
}

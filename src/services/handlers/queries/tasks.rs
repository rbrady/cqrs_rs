use crate::adapters::tasks::TaskRepository;
use crate::domain::models::tasks::Task;

pub fn list_tasks_handler() -> &Vec<Task> {

    let mut repository = TaskRepository::new();

    repository.list()
}
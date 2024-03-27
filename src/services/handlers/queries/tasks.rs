use crate::adapters::tasks::TaskRepository;
use crate::domain::models::tasks::Task;

pub fn list_tasks_handler(tasks: &mut Vec<Task>) -> Vec<Task> {
    let mut repository = TaskRepository::new(tasks);
    repository.list()
}

use crate::domain::commands::tasks::CreateTaskCommand;
use crate::domain::models::tasks::Task;
use crate::adapters::tasks::TaskRepository;


pub fn create_task_handler(command: CreateTaskCommand) -> Task {
    let mut repository = TaskRepository::new();

    // Example of storing a task
    let new_task = repository.create(Task {
        id: None,
        title: command.title,
        description: command.description,
        assigner: command.assigner,
        due_date: command.due_date,
    });

    new_task
}
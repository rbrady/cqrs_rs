use crate::domain::commands::tasks::CreateTaskCommand;
use crate::domain::models::tasks::Task;
use crate::adapters::tasks::TaskRepository;

pub fn create_task_handler(command: CreateTaskCommand, tasks: &mut Vec<Task>) -> Task {
    let mut repository = TaskRepository::new(tasks);

    // Create a task
    let new_task = Task {
        id: None,
        title: command.title,
        description: command.description,
        assigner: command.assigner,
        due_date: command.due_date,
    };

    // Example of storing a task in a repository (if needed)
    repository.create(new_task.clone());

    // Return the newly created task
    new_task
}

extern crate chrono;
mod domain; // This line declares the domain module
mod services; // This line declares the services module
mod adapters;

use chrono::NaiveDate;
use crate::services::tasks::TaskService;
use crate::domain::models::tasks::Task;


fn main() {
    let mut in_mem_db:Vec<Task> = Vec::new();
    let mut task_service = TaskService::new(&mut in_mem_db);

    // Add 3 tasks
    task_service.create(
        "Task 1".to_string(),
        "Description for Task 1".to_string(),
        "Assigner A".to_string(),
        NaiveDate::from_ymd(2024, 3, 15),
    );

    task_service.create(
        "Task 2".to_string(),
        "Description for Task 2".to_string(),
        "Assigner B".to_string(),
        NaiveDate::from_ymd(2024, 4, 20),
    );

    task_service.create(
        "Task 3".to_string(),
        "Description for Task 3".to_string(),
        "Assigner C".to_string(),
        NaiveDate::from_ymd(2024, 5, 25),
    );

    // List and print the tasks
    let tasks = task_service.list();
    for task in tasks {
        println!("Task: {}, {}, {}, {}", task.title, task.description, task.assigner, task.due_date);
    }

}

use crate::domain::models::tasks::Task;

pub struct TaskRepository {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskRepository {
    pub fn new() -> Self {
        TaskRepository { tasks: Vec::new(), next_id: 1 }
    }

    pub fn create(&mut self, mut task: Task) -> Task {
        task.id = Some(self.next_id);
        self.next_id += 1;
        self.tasks.push(task);
        task
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|&task| task.id == id)
    }

    pub fn update(&mut self, updated_task: Task) -> Option<()> {
        let pos = self.tasks.iter().position(|task| task.id == updated_task.id)?;
        self.tasks[pos] = updated_task;
        Some(())
    }

    pub fn delete(&mut self, id: u32) -> Option<()> {
        let pos = self.tasks.iter().position(|task| task.id == id)?;
        self.tasks.remove(pos);
        Some(())
    }

    pub fn list(&self) -> &Vec<Task> {
        &self.tasks
    }
}

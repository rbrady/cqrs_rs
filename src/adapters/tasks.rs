use crate::domain::models::tasks::Task;

pub struct TaskRepository<'a> {
    tasks: &'a mut Vec<Task>,
    next_id: u32,
}

impl<'a> TaskRepository<'a> {
    pub fn new(tasks: &'a mut Vec<Task>) -> Self {
        Self {
            tasks, next_id: 1
        }
    }

    pub fn create(&mut self, mut task: Task) -> Task {
        task.id = Option::from(self.next_id);
        self.next_id += 1;
        self.tasks.push(task.clone());
        task
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
       self.tasks.iter().find(|&task| task.id == Option::from(id))
   }

    pub fn update(&mut self, updated_task: Task) -> Option<()> {
        let pos = self.tasks.iter().position(|task| task.id == updated_task.id)?;
        self.tasks[pos] = updated_task;
        Some(())
    }

    pub fn delete(&mut self, id: u32) -> Option<()> {
        let pos = self.tasks.iter().position(|task| task.id == Option::from(id))?;
        self.tasks.remove(pos);
        Some(())
    }

    pub fn list(&mut self) -> Vec<Task> {
        self.tasks.clone()
    }
}

use std::fmt::Display;

#[derive(Debug)]
pub struct Task {
    priority: i32,
    value: String,
    is_completed: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PRIORITY: {}, TASK: {}, COMPLETED: {}",
            self.priority, self.value, self.is_completed
        )
    }
}

pub struct AppState {
    pub tasks: Vec<Task>,
}

impl AppState {
    pub fn add_task(&mut self, value: String, priority: Option<i32>) {
        let p = priority.unwrap_or(10);
        self.tasks.push(Task {
            priority: p,
            value,
            is_completed: false,
        })
    }

    pub fn remove_last_task(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    pub fn list_tasks(&mut self) {
        for task in self.tasks.iter() {
            println!("{}", task);
        }
    }
}

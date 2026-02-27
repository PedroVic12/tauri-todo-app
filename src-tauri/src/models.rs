// Task struct representing a task in the application.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

// Category struct representing a category for tasks.
#[derive(Debug, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub tasks: Vec<Task>,
}
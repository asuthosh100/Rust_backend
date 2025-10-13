


// Instructions: Define an enum TaskType  with variants: Computation , IO , and Network . 
// Create a trait Task  with fn run(&self) -> String . 
// Implement Task  for a few struct types, each corresponding to a TaskType . 
// Write a function that accepts a Box<dyn Task>  and executes it.


enum TaskType {
    Computation,
    IO,
    Network,
}



trait Task {
    fn run(&self) -> String;
}

impl Task for TaskType {
    fn run(&self) -> String {
        match self {
            TaskType::Computation => "Running Computation Task".to_string(),
            TaskType::IO => "Running IO Task".to_string(),
            TaskType::Network => "Running Network Task".to_string(),
        }
    }
}

fn run_task(task: Box<dyn Task>) -> String {
    task.run()
}

fn main() {
    let t1 = Box::new(TaskType::Computation);
    let t2 = Box::new(TaskType::IO);
    let t3 = Box::new(TaskType::Network);

    println!("{}", run_task(t1));
    println!("{}", run_task(t2));
    println!("{}", run_task(t3));
}




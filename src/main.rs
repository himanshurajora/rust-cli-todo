use std::{io::{stdin, stdout, Write}, process::exit};

use rust_cli_todo::AppState;

fn main() {
    let mut app_state = AppState {
        tasks: vec![]
    };

    app_state.add_task("Sample Task".to_string(), None);
    application_loop(&mut app_state);
}

fn application_loop(app_state: &mut AppState) {
    let mut input = String::new();
    print!("Choose from the Options \n 1. List All Tasks \n 2. Add New Task \n 3. Remove Task By Id \n 4. Mark the task completed \n Press anything else to exit \n");
    // TODO: Later a function for reading input
    stdin().read_line(&mut input).ok().expect("Failed to read input");

    println!("-------------------------");
    match input.as_str().trim() {
      "1" => app_state.list_tasks(), 
      "2" => add_task(app_state),
       _ => {
            println!("Thanks For Using Our App!!")
        }
    }

    println!("-------------------------");
    application_loop(app_state);
}

fn add_task(app_state: &mut AppState) {
    let mut task = String::new();
    let mut priority = String::new();

    // read task and priority
    print!("Please Enter Task: ");
    stdout().flush().expect("Couldn't flush");
    stdin().read_line(&mut task).ok().expect("Can't read task");
    print!("\n Please Enter Priority");
    stdout().flush().expect("Couldn't flush");
    stdin().read_line(&mut priority).ok().expect("Can't read priority");

    app_state.add_task(task, Some(priority.trim().parse::<i32>().expect("Could not parse")));

    println!("Added the task successfully");
}

fn remove_task(app_state: &mut AppState, id: u16) {
    todo!()
}


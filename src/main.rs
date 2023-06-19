use rust_cli_todo::{ AppState};

fn main() {
    let mut app_state = AppState {
        tasks: vec![]
    };


    app_state.add_task("Test Task".to_string(), Some(0));

    println!("{}", app_state.tasks.get(0).unwrap())   ;
    application_loop(app_state);
}

fn application_loop(_app_state: AppState) {
    
}


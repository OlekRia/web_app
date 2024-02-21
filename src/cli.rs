// mod processes;
// mod state;
// mod to_do;

// use processes::process_input;
// use state::read_file;
// use std::env;
// use to_do::{data::FILE_NAME, enums::TaskStatus, to_do_factory};

// fn grab_args() -> (String, String) {
//     let args: Vec<String> = env::args().collect();
//     let command: &String = &args[1];
//     let title: &String = &args[2];

//     (command.to_string(), title.to_string())
// }

// fn run_cli() {
//     let (command, title) = grab_args();
//     let state = read_file(FILE_NAME);

//     let status: String = match state.get(&title) {
//         Some(result) => result.to_string().replace("\"", ""),
//         None => "pending".to_owned(),
//     };

//     let item = to_do_factory(&title, TaskStatus::from_string(status.to_uppercase()));
//     process_input(item, command, &state);
// }

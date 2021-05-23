use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("jqs> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_input(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn handle_input(a: &str) {
    let cmd = get_cmd(a);
    match cmd {
        Some(f) => f(),
        None => {
            println!("Invalid command");
            print_usage();
        }
    }
}

fn get_cmd(a: &str) -> Option<fn() -> ()> {
    let a = a.trim();
    match a {
        "help" => Some(print_usage),
        _ => None,
    }
}

fn print_usage() {
    println!("Commands:");
    println!("  help");
}

use std::env;
use std::process::exit;
use std::process::ExitCode;

fn usage() {
    println!("usage");
}

fn list_sessions() -> ExitCode {
    println!("list_sessions");
    ExitCode::SUCCESS
}

fn create_session() {
    println!("create_session");
}

fn attach_session() {
    println!("attach_session");
}

fn session_exists() -> bool {
    println!("session_exists");
    false
}

fn main() -> ExitCode {
    let mut args = env::args();
    let _program = args.next().expect("program name");
    let args = args.collect::<Vec<_>>();

    if args.is_empty() {
        return list_sessions();
    }
    let mut action = ' ';

    for arg in args {
        match arg.as_str() {
            "-a" => action = 'a',
            "-A" => action = 'A',
            "-c" => action = 'c',
            "-n" => action = 'n',
            "-v" => {
                println!("abduco-0.0.1 © 2024 Syed Fasiuddin");
                exit(0);
            }
            _ => usage(),
        }
    }

    match action {
        'A' => {
            if !session_exists() {
                create_session();
            }
            attach_session();
        }
        'a' => {
            if !session_exists() {
                eprintln!("error: session does not exist");
                return ExitCode::FAILURE;
            }
            attach_session();
        }
        'c' => {
            if session_exists() {
                eprintln!("error: session already exist");
                return ExitCode::FAILURE;
            }
            create_session();
            attach_session();
        }
        'n' => {
            if session_exists() {
                eprintln!("error: session already exist");
                return ExitCode::FAILURE;
            }
            create_session();
        }
        x => unreachable!("unknown action {x}"),
    }

    ExitCode::SUCCESS
}

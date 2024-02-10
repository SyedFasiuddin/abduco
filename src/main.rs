#![allow(unused_variables, unused_assignments)]

use std::env;
use std::process::ExitCode;

macro_rules! CTRL {
    ( $ch:expr ) => {
        $ch as u8 & 0x1F
    }
}

fn usage() -> ExitCode {
    println!("usage: abduco [-a|-A|-c|-n] [-r] [-l] [-f] [-e detachkey] name command");
    ExitCode::FAILURE
}

fn list_sessions() -> ExitCode {
    println!("list_sessions");
    ExitCode::SUCCESS
}

fn create_session(session_name: &str) {
    println!("create_session: {session_name}");
}

fn attach_session(session_name: &str) {
    println!("attach_session: {session_name}");
}

fn session_exists(session_name: &str) -> bool {
    println!("session_exists: {session_name}");
    false
}

fn main() -> ExitCode {
    let mut args = env::args();
    let _program = args.next().expect("program name");
    let mut args = args.collect::<Vec<_>>();
    args.reverse();

    if args.is_empty() {
        return list_sessions();
    }

    let mut action = ' ';
    let mut force = false;
    let mut passthrough = false;
    let mut quiet = false;

    let mut key_detach = CTRL!('\\');
    let mut handle_e = false;

    while let Some(arg) = args.pop() {
        if !arg.starts_with("-") {
            if handle_e {
                let mut chars = arg.chars();
                if let Some(x) = chars.next()  {
                    if x == '^' {
                        if let Some(x) = chars.next() {
                            key_detach = CTRL!(x);
                        }
                    } else {
                        key_detach = x as u8;
                    }
                }
                handle_e = false;
            }

            args.push(arg);
            break;
        }
        let mut chars = arg.chars();
        while let Some(ch) = chars.next() {
            match ch {
                '-' => continue,
                'a' => action = 'a',
                'A' => action = 'A',
                'c' => action = 'c',
                'n' => action = 'n',
                'e' => {
                    if let Some(x) = chars.next() {
                        if x == '^' {
                            if let Some(x) = chars.next() {
                                key_detach = CTRL!(x);
                            }
                        } else {
                            key_detach = x as u8;
                        }
                    } else {
                        handle_e = true;
                    }
                    break;
                }
                'f' => force = true,
                'p' => passthrough = true,
                'q' => quiet = true,
                'r' => todo!("readonly"),
                'l' => todo!("low priority"),
                'v' => {
                    let version = env!("CARGO_PKG_VERSION");
                    println!("abduco-{version} © 2024 Syed Fasiuddin");
                    return ExitCode::SUCCESS;
                }
                x => return usage(),
            }
        }
    }

    let mut session_name = String::new();
    if let Some(name) = args.pop() {
        session_name = name;
    }
    let mut command = String::new();
    if let Some(name) = args.pop() {
        command = name;
    }
    args.reverse();

    match action {
        'A' => {
            if !session_exists(&session_name) {
                create_session(&session_name);
            }
            attach_session(&session_name);
        }
        'a' => {
            if !session_exists(&session_name) {
                eprintln!("error: session does not exist");
                return ExitCode::FAILURE;
            }
            attach_session(&session_name);
        }
        'c' => {
            if session_exists(&session_name) {
                eprintln!("error: session already exist");
                return ExitCode::FAILURE;
            }
            create_session(&session_name);
            attach_session(&session_name);
        }
        'n' => {
            if session_exists(&session_name) {
                eprintln!("error: session already exist");
                return ExitCode::FAILURE;
            }
            create_session(&session_name);
        }
        _ => return usage(),
    }

    ExitCode::SUCCESS
}

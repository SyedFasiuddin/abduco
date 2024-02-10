#![allow(unused_variables, unused_assignments, dead_code)]

use std::env;
use std::path::Path;
use std::process::ExitCode;

macro_rules! CTRL {
    ( $ch:expr ) => {
        $ch as u8 & 0x1F
    };
}

const ABDUCO_CMD: &'static str = "dvtm";

struct Dir {
    path: Box<Path>,
    personal: bool,
}

impl Dir {
    fn form_env(env: &str, personal: bool) -> Self {
        let path = env::var(env).unwrap_or("".to_string());
        Self {
            path: Path::new(&path).into(),
            personal,
        }
    }

    fn from_path(path: &str, personal: bool) -> Self {
        Self {
            path: Path::new(&path).into(),
            personal,
        }
    }
}

fn usage() -> ExitCode {
    println!("usage: abduco [-a|-A|-c|-n] [-r] [-l] [-f] [-e detachkey] name command");
    ExitCode::FAILURE
}

fn list_sessions(socket_dirs: &[Dir]) -> ExitCode {
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
    let socket_dirs: [Dir; 4] = [
        Dir::form_env("ABDUCO_SOCKET_DIR", false),
        Dir::form_env("HOME", true),
        Dir::form_env("TMPDIR", false),
        Dir::from_path("/tmp", false),
    ];

    if args.is_empty() {
        return list_sessions(&socket_dirs);
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
                if let Some(x) = chars.next() {
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

    let session_name = match args.pop() {
        Some(name) => name,
        None => return usage(),
    };

    let command = match args.pop() {
        Some(name) => name,
        None => ABDUCO_CMD.to_string(),
    };
    if !args.is_empty() {
        args.reverse();
    }

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

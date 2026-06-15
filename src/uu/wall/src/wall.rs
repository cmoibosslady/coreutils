// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::builder::ValueParser;
use clap::parser::ValuesRef;
use clap::{Arg, ArgAction, Command};
use thiserror::Error;
use std::env;
use std::ffi::OsString;
use std::io;
use std::io::prelude::*;
use std::string::FromUtf8Error;

use uucore::error::{UResult, UError};
use uucore::format_usage;
use uucore::utmpx::Utmpx;

use uucore::translate;
const STRING: &str = "string";
const OPT_GROUP: &str = "group";
const OPT_NOBANNER: &str = "nobanner";
const OPT_TIMEOUT: &str = "timeout";

#[derive(Error, Debug)]
enum WallError {
    #[error("{}", translate!("wall-error-stdin"))]
    Stdin(#[from] io::Error),
    #[error("{}", translate!("wall-encoding-error"))]
    VecToString(#[from] FromUtf8Error),
    #[error("{}", translate!("wall-error-osstring"))]
    ToStringError,
    #[error("{}", translate!("wall-error-mac-os-too-many-args"))]
    MacOsTooManyArgs,
}

impl UError for WallError {
    fn code(&self) -> i32 {
        // change this to watch wall error codes?
        // match self {
        //     WallError::Stdin(_) => 1,
        //     WallError::VecToString(_) => 1,
        //     WallError::ToStringError => 1,
        //     WallError::MacOsTooManyArgs => 16 or 1,
        // }
        1
    }
}

#[uucore::main(no_signals)]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let args = args.skip(1).peekable();
    let matches = uucore::clap_localization::handle_clap_result(uu_app(), args)?;
    let message = get_message(matches.get_many(STRING).unwrap_or_default())?;
    let users = find_logged_users()?;
    write_to_terminals(message, users)?;
    Ok(())
}

pub fn uu_app() -> Command {
    Command::new("wall")
        .version(uucore::crate_version!())
        .about(translate!("wall-about"))
        .override_usage(format_usage(&translate!("pwd-usage")))
        .arg(
            Arg::new(OPT_GROUP)
                .short('g')
                .long(OPT_GROUP)
                .value_name("GROUP")
                .help(translate!("wall-help-group"))
                .num_args(1)
                .action(ArgAction::Append) // User can target more than one group
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new(OPT_NOBANNER)
                .short('n')
                .long(OPT_NOBANNER)
                .action(ArgAction::SetTrue)
                .help(translate!("wall-help-nobanner"))
        )
        .arg(
            Arg::new(OPT_TIMEOUT)
                .short('t')
                .long(OPT_TIMEOUT)
                .value_name("SECONDS")
                .help(translate!("wall-help-timeout"))
                .num_args(1)
        )
        .arg(
            Arg::new(STRING)
                .action(ArgAction::Append)
                .value_parser(ValueParser::os_string())
        )
}


fn get_message(args: ValuesRef<OsString>) -> Result<String, WallError> {
    if args.len() == 0 {
        read_from_stdin()
    } else {
        if args.len() == 1 {
            read_from_file(args.into_iter().next().unwrap())
        } else {
            if cfg!(target_os = "macos") {
                Err(WallError::MacOsTooManyArgs)
            } else {
                concatenate_message(args)
            }
        }
    }
}

fn read_from_stdin() -> Result<String, WallError> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    let res = String::from_utf8(buffer)?;
    Ok(res)
}

fn read_from_file(file: &OsString) -> Result<String, WallError> {
    let mut buffer = Vec::new();
    let mut file = std::fs::File::open(file)?;
    file.read_to_end(&mut buffer)?;
    let res = String::from_utf8(buffer)?;
    Ok(res)
}

fn concatenate_message(args: ValuesRef<OsString>) -> Result<String, WallError> {
    let mut res = String::new();
    for arg in args {
        res.push_str(arg.to_str().ok_or(WallError::ToStringError)?);
        res.push(' ');
    }
    res.pop();
    Ok(res)

}

fn find_logged_users() -> Result<Vec<String>, WallError> {
    let mut res = Vec::<String>::new();
    for ut in Utmpx::iter_all_records() {
        if ut.is_user_process() { // it's a user's tty
            let tty_path = String::from("/dev/") +
                &ut.tty_device().to_string();
            res.push(tty_path);
        }
    }
    Ok(res)
}

fn wall_intro_message() -> String {
    // retreive user + hostname from terminal
    // retreive date
    let home = "USER";
    let hostname = "HOSTNAME";
    let home = env::var_os(home).unwrap_or_default();
    let hostname = env::var_os(hostname).unwrap_or_default();
    let tty = String::from("/dev/what>"); // can take it from uucore ? -> tty.rs already coded
    let date = String::from("MONDAY"); // date.rs exist in uucore
    format!("Broadcast message from {}@{} ({}) ({})\n\n",
        home.to_string_lossy(),
        hostname.to_string_lossy(),
        tty,
        date)
}

fn write_to_terminals(message: String, users: Vec<String>) -> UResult<()> {
    let transmission = wall_intro_message() + &message;
    for user in users {
        let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .open(user) {
                Ok(f) => f,
                Err(e) => {
                eprintln!("{}: {}", translate!("wall-error-open-terminal"), e);
                continue;
                }
            };
        file.write_all(transmission.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use clap::parser::ValuesRef;
    use crate::{uu_app, get_message, find_logged_users, write_to_terminals};
    use crate::{OPT_GROUP, STRING};
    use std::ffi::OsString;
    use std::process::{Command, Output};

    #[test]
    fn test_basic_clap_implementation() {
        let group = String::from("staff");
        let file = String::from("LICENSE");
        let command = vec!("wall", "-g", &group, &file);
        let matches = uucore::clap_localization::handle_clap_result(uu_app(), command).expect("Error
            outside of test perimeter");
        assert!(matches.get_one::<String>(OPT_GROUP).unwrap() == &group);
        assert!(matches.get_one::<OsString>(STRING).unwrap().clone().into_string().unwrap() == file);
    }

    #[test]
    fn test_get_message_on_file() {
        let file = String::from("LICENSE");

        // wall does not print the content of the file in the stdout, it sends it to the tty(s)
        // Hence the use of cat to check if the get_message function can extract correctly the
        // file
        let mut command = Command::new("cat");
        command.arg(&file);
        let output: Output = match command.output() {
            Ok(o) => o,
            Err(_) => panic!("Failed to start 'cat' command")
        };
        if !output.status.success() {
            panic!("'cat' command exit with failure status")
        }
        let command_output = match String::from_utf8(output.stdout) {
            Ok(o) => o,
            Err(_) => panic!("Failed to convert 'cat' output")
        };

        let command = vec!("wall", &file);
        let matches = uucore::clap_localization::handle_clap_result(uu_app(),
        command).expect("External error");
        let pos_arg = match matches.get_many(STRING) {
            Some(o) => o,
            None => ValuesRef::<OsString>::default(),
        };
        let function_output = get_message(pos_arg).unwrap();
        assert_eq!(function_output, command_output);
    }

    #[test]
    fn test_get_message_on_stdin() {
        // for the moment test against cat is not implemented
        let command = vec!("wall");
        let matches = uucore::clap_localization::handle_clap_result(uu_app(),
        command).expect("External error");
        let pos_arg = match matches.get_many(STRING) {
            Some(o) => o,
            None => ValuesRef::<OsString>::default(),
        };
        let function_output = get_message(pos_arg).unwrap();
        assert_eq!(function_output, "Hello !\n");
    }

    #[test]
    fn test_arguments_as_message() {
        let command = vec!("wall", "Hello", "World", "!");
        let matches = uucore::clap_localization::handle_clap_result(uu_app(),
        command).expect("External error");
        let pos_arg = match matches.get_many(STRING) {
            Some(o) => o,
            None => ValuesRef::<OsString>::default(),
        };
        let function_output = get_message(pos_arg).unwrap();
        assert_eq!(function_output, "Hello World !");
    }

    #[test]
    fn test_found_connected_users() {
        let users = find_logged_users().unwrap();
        assert_eq!(users, vec!(String::from("tty1"), String::from("tty2"), String::from("tty3")));
    }

    #[test]
    fn test_print_to_terminals() {
        let users = find_logged_users().unwrap();
        let result = write_to_terminals(String::from("hello world!"), users);
    }
}


// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::builder::ValueParser;
use clap::parser::ValuesRef;
use clap::{Arg, ArgAction, Command};
use thiserror::Error;
use std::ffi::OsString;
use std::io;
use std::io::prelude::*;
use std::string::FromUtf8Error;

use uucore::error::UResult;
use uucore::format_usage;

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
}

#[uucore::main(no_signals)]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let args = args.skip(1).peekable();
    let _matches = uucore::clap_localization::handle_clap_result(uu_app(), args)?;
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
            concatenate_message(args)
        }
        // if not macOS print message
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

// fn find_logged_users() -> UResult<()> {
//     Ok(())
// }

// fn write_to_terminals() -> UResult<()> {
//     Ok(())
// }

#[cfg(test)]
mod tests {

    use clap::parser::ValuesRef;
    use crate::{uu_app, get_message};
    use crate::{OPT_GROUP, STRING};
    use std::ffi::OsString;
    use std::io::Write;
    use std::process::{Command, Output, Stdio};

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

        // let testing_message = "Hello !\n";
        // let mut binding = Command::new("cat");
        // let mut cat_command = binding.stdin(Stdio::piped());
        // let mut child = cat_command.spawn().expect("Cannot init 'cat' command");

        // if let Some(mut stdin) = child.stdin.take() {
        //     stdin.write_all(testing_message.as_bytes())
        //         .expect("Cannot write into pipe");
        // }
        // drop(child.stdin);

        // let output: Output = child.wait_with_output()
        //     .expect("Failed to wait for cat process");
        // if !output.status.success() {
        //     panic!("'cat' command exit with failure status")
        // }
        // let command_output = match String::from_utf8(output.stdout) {
        //     Ok(o) => o,
        //     Err(_) => panic!("Failed to convert 'cat' output")
        // };

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
}


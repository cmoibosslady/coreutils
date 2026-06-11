// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::builder::ValueParser;
use clap::{Arg, ArgAction, Command};
use uucore::error::UResult;
use uucore::format_usage;

use uucore::translate;
const STRING: &str = "string";
const OPT_GROUP: &str = "group";
const OPT_NOBANNER: &str = "nobanner";
const OPT_TIMEOUT: &str = "timeout";

#[uucore::main(no_signals)]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {

    let mut args = args.skip(1).peekable();
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


fn get_message(args: impl uucore::Args) -> UResult<()> {

}

fn find_logged_users() -> UResult<()> {

}

fn write_to_terminals() -> UResult<()> {

}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, stdout};

    #[test]
    fn test_write_to_terminals() {
        let mut writer = BufWriter::new(stdout());
        // Here you would call the function that writes to terminals
        // and assert the expected output.
    }

    #[test]
    fn test_find_logged_users() {
        // Here you would call the function that finds logged users
        // and assert the expected output.
    }

    #[test]
    fn test_get_message() {
        // Here you would call the function that gets the message
        // and assert the expected output.
    }
}


// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{Arg, Command};
use std::io;
use uucore::{FromIo, UResult};

use uucore::translate;

const FILE: &str = "file";
const OPT_GROUP: &str = "group";
const OPT_NOBANNER: &str = "nobanner"
const OPT_TIMEOUT: &str = "timeout"

pub fn get_message() -> UResult<()> {

}

pub fn find_logged_users() -> UResult<()> {

}

fn write_to_terminals() -> UResult<()> {

}

#[uucore::main(no_signals)]
pub fn uumain(args:: impl uucore::Args) -> UResult<()> {

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
                .help(translate!("wall-help-group"))
        )
        .arg(
            Arg::new(OPT_NOBANNER)
                .short('n')
                .long(OPT_NO_BANNER)
                .help(translate!("wall-help-nobanner"))
        )
        .arg(
            Arg::new(OPT_TIMEOUT)
                .short('t')
                .long(OPT_TIMEOUT)
                .help(translate!("wall-help-timeout"))
        )
        .arg(
            Arg::new(options::FILE)
        )
}

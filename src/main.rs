#[macro_use]
extern crate clap;

mod icurl;
mod network;

use clap::{App, Arg, ArgGroup};
fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("[url] 'Set a target url'"))
        .args_from_usage(
            "--GET 'Set a http GET mehod'
            --POST 'Set a http POST method'",
        )
        .group(ArgGroup::with_name("method").args(&["GET", "POST"]))
        .arg(Arg::from_usage("[highlight] --hi 'Use a syntax highlight'"))
        .get_matches();

    let mut icurl = icurl::Icurl::new(matches);
    icurl.stack_action();
    icurl.run_action();
}

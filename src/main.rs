#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgGroup};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    if let Some(url) = matches.value_of("url") {
        let resp = reqwest::get(url).await?;

        let body = resp.text().await?.to_string();
        display_response(body, matches.is_present("highlight"));
    }
    Ok(())
}

fn display_response(body: String, is_highlight: bool) {
    if is_highlight {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_by_extension("html").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        for line in LinesWithEndings::from(&body) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }
    } else {
        for line in LinesWithEndings::from(&body) {
            print!("{}", line);
        }
    }
}

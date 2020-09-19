use crate::icurl::{HttpMethod, Icurl};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn request(icurl: &Icurl) {
    let body = match icurl.state.method {
        Some(HttpMethod::GET) => get_request(icurl),
        Some(HttpMethod::POST) => post_request(icurl),
        _ => panic!("Undifiend http method"),
    };

    let res = match body {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    };

    if icurl.state.is_highlight {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        let syntax = syntax_set.find_syntax_by_extension("html").unwrap();
        let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);
        for line in LinesWithEndings::from(&res) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &syntax_set);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }
    } else {
        for line in LinesWithEndings::from(&res) {
            print!("{}", line);
        }
    }
}

#[tokio::main]
async fn get_request(icurl: &Icurl) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&icurl.state.url.clone().unwrap()).await?;
    let body = resp.text().await?.to_string();
    Ok(body)
}

#[tokio::main]
async fn post_request(icurl: &Icurl) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .post(&icurl.state.url.clone().unwrap())
        .json(&icurl.state.request_body)
        .send()
        .await?;

    let body = resp.text().await?.to_string();
    Ok(body)
}

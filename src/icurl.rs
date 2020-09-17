use clap::ArgMatches;
use std::collections::VecDeque;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    fn new(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Action {
    SET_URL,
    SET_HTTP_METHOD,
    SET_THEME,
}

struct State {
    url: Option<String>,
    method: Option<HttpMethod>,
    is_highlight: bool,
}

impl State {
    pub fn new(matches: ArgMatches) -> State {
        let url = matches.value_of("url").map(String::from);

        // TODO: Need Refactoring
        let method: Option<HttpMethod> = if matches.is_present("method") {
            if matches.is_present("GET") {
                Some(HttpMethod::GET)
            } else if matches.is_present("POST") {
                Some(HttpMethod::POST)
            } else {
                None
            }
        } else {
            None
        };

        let is_highlight = matches.is_present("highlight");

        State {
            url,
            method,
            is_highlight,
        }
    }
}

pub struct Icurl {
    state: State,
    stack: VecDeque<Action>,
    syntax_set: Option<SyntaxSet>,
    theme_set: Option<ThemeSet>,
}

impl Icurl {
    pub fn new(matches: ArgMatches) -> Icurl {
        Icurl {
            state: State::new(matches),
            stack: VecDeque::new(),
            syntax_set: None,
            theme_set: None,
        }
    }

    pub fn stack_action(&mut self) {
        if self.state.is_highlight {
            self.stack.push_back(Action::SET_THEME);
        }

        if self.state.url.is_none() {
            self.stack.push_back(Action::SET_URL);
        }

        if self.state.method.is_none() {
            self.stack.push_back(Action::SET_HTTP_METHOD);
        }
    }

    /// Execute the actions in the stack one after the other.
    pub fn run_action(&mut self) {
        loop {
            let action = self.pop_front_action();

            match action {
                Some(Action::SET_THEME) => self.set_theme(),
                Some(Action::SET_URL) => self.set_url(),
                Some(Action::SET_HTTP_METHOD) => self.set_http_method(),
                _ => break,
            }
        }

        self.request();
    }

    fn set_url(&mut self) {
        println!("> Please input access url");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();

        self.state.url = Some(answer);
        println!();
    }

    fn set_http_method(&mut self) {
        println!("> Set a http method");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim();

        self.state.method = HttpMethod::new(answer);
        println!();
    }

    fn set_theme(&mut self) {
        self.syntax_set = Some(SyntaxSet::load_defaults_newlines());
        self.theme_set = Some(ThemeSet::load_defaults());
    }

    #[tokio::main]
    async fn request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(&self.state.url.clone().unwrap()).await?;
        let body = resp.text().await?.to_string();

        if self.state.is_highlight && self.syntax_set.is_some() && self.theme_set.is_some() {
            let syntax_set = std::mem::replace(&mut self.syntax_set, None).unwrap();
            let theme_set = std::mem::replace(&mut self.theme_set, None).unwrap();

            let syntax = syntax_set.find_syntax_by_extension("html").unwrap();
            let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);
            for line in LinesWithEndings::from(&body) {
                let ranges: Vec<(Style, &str)> = h.highlight(line, &syntax_set);
                let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                print!("{}", escaped);
            }
        } else {
            for line in LinesWithEndings::from(&body) {
                print!("{}", line);
            }
        }
        Ok(())
    }

    pub fn pop_front_action(&mut self) -> Option<Action> {
        self.stack.pop_front()
    }

    pub fn pop_back_action(&mut self) -> Option<Action> {
        self.stack.pop_back()
    }

    pub fn push_front_action(&mut self, action: Action) {
        self.stack.push_front(action);
    }

    pub fn push_back_action(&mut self, action: Action) {
        self.stack.push_back(action);
    }
}

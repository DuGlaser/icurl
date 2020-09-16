use clap::ArgMatches;
use std::collections::VecDeque;

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    fn new(method: &str) -> Option<HttpMethod> {
        println!("HttpMethod new");
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
}

impl Icurl {
    pub fn new(matches: ArgMatches) -> Icurl {
        Icurl {
            state: State::new(matches),
            stack: VecDeque::new(),
        }
    }

    pub fn stack_action(&mut self) {
        if self.state.url.is_none() {
            self.stack.push_back(Action::SET_URL);
        }

        if self.state.method.is_none() {
            self.stack.push_back(Action::SET_HTTP_METHOD);
        }
    }

    pub fn run_action(&mut self) {
        loop {
            let action = self.pop_front_action();

            match action {
                Some(Action::SET_URL) => self.set_url(),
                Some(Action::SET_HTTP_METHOD) => self.set_http_method(),
                _ => break,
            }
        }
    }

    fn set_url(&mut self) {
        println!("Please input access url: ");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();

        self.state.url = Some(answer);

        println!("{:?}", self.state.url);
    }

    fn set_http_method(&mut self) {
        println!("Set a http method: ");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim();

        self.state.method = HttpMethod::new(answer);

        println!("{:?}", self.state.method);
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

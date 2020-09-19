use crate::network::request;

use clap::ArgMatches;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum HttpMethod {
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
    SET_REQUEST_BODY,
}

pub struct State {
    pub url: Option<String>,
    pub method: Option<HttpMethod>,
    pub is_highlight: bool,
    pub request_body: HashMap<String, String>,
}

impl State {
    pub fn new(matches: ArgMatches) -> State {
        let url = matches.value_of("url").map(String::from);

        // TODO: Need Refactoring
        let method: Option<HttpMethod> = if matches.is_present("method") {
            if matches.is_present("GET") {
                HttpMethod::new("GET")
            } else if matches.is_present("POST") {
                HttpMethod::new("POST")
            } else {
                None
            }
        } else {
            None
        };

        let is_highlight = matches.is_present("highlight");
        let request_body = HashMap::new();

        State {
            url,
            method,
            is_highlight,
            request_body,
        }
    }
}

pub struct Icurl {
    pub state: State,
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
        } else {
            match self.state.method {
                Some(HttpMethod::POST) => self.stack.push_back(Action::SET_REQUEST_BODY),
                _ => println!("Undifiend http method"),
            }
        }
    }

    /// Execute the actions in the stack one after the other.
    pub fn run_action(&mut self) {
        loop {
            let action = self.pop_front_action();

            match action {
                Some(Action::SET_URL) => self.set_url(),
                Some(Action::SET_HTTP_METHOD) => self.set_http_method(),
                Some(Action::SET_REQUEST_BODY) => self.set_request_body(),
                _ => break,
            }
        }

        request(self);
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
        let answer = word.trim().to_uppercase();

        if answer == "POST" {
            self.push_front_action(Action::SET_REQUEST_BODY);
        }

        self.state.method = HttpMethod::new(&answer);
        println!();
    }

    fn set_request_body(&mut self) {
        println!("> Please input key");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let key = word.trim().to_string();

        println!();
        println!("> Please input value");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let value = word.trim().to_string();

        self.state.request_body.insert(key, value);

        println!();
    }

    fn pop_front_action(&mut self) -> Option<Action> {
        self.stack.pop_front()
    }

    fn pop_back_action(&mut self) -> Option<Action> {
        self.stack.pop_back()
    }

    fn push_front_action(&mut self, action: Action) {
        self.stack.push_front(action);
    }

    fn push_back_action(&mut self, action: Action) {
        self.stack.push_back(action);
    }
}

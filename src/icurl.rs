use clap::ArgMatches;
use std::collections::VecDeque;

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
}

struct State {
    url: Option<String>,
    method: Option<HttpMethod>,
    is_highlight: bool,
}

impl State {
    pub fn new(matches: ArgMatches) -> State {
        let url = matches.value_of("url").map(String::from);

        let method: Option<HttpMethod> = match matches.value_of("method") {
            Some(method) => HttpMethod::new(method),
            _ => None,
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

    pub fn pop_front_action(&mut self) {
        self.stack.pop_front();
    }

    pub fn pop_back_action(&mut self) {
        self.stack.pop_back();
    }

    pub fn push_front_action(&mut self, action: Action) {
        self.stack.push_front(action);
    }

    pub fn push_back_action(&mut self, action: Action) {
        self.stack.push_back(action);
    }
}

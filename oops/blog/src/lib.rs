pub struct Post<'a> {
    text: &'a str,
    post_state: State,
}

enum State {
    Draft,
    WaitingForApproval,
    Approved,
}

impl<'a> Default for Post<'a> {
    fn default() -> Self {
        Self {
            text: "",
            post_state: State::Draft,
        }
    }
}

impl<'a> Post<'a> {
    pub fn new() -> Self {
        Post::default()
    }
    pub fn add_text(&mut self, text: &'a str) {
        match self.post_state {
            State::Draft => {
                self.text = text;
                self.post_state = State::WaitingForApproval;
            }
            _ => panic!("Can't add text when not in draft state"),
        };
    }
    pub fn approve(&mut self) {
        match self.post_state {
            State::WaitingForApproval => self.post_state = State::Approved,
            _ => panic!("Can't approve, coz state is not in waiting for approval"),
        }
    }

    pub fn content(&self) -> &str {
        match self.post_state {
            State::Approved => self.text,
            _ => "",
        }
    }
}

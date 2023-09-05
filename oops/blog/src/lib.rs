trait State {
    fn show_content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(&self) -> Box<dyn State>;

    fn approve(&self) -> Box<dyn State>;
}

pub struct Draft;

impl State for Draft {
    fn approve(&self) -> Box<dyn State> {
        Box::new(Self)
    }
    fn request_review(&self) -> Box<dyn State> {
        Box::new(WaitingForApproval)
    }
}

pub struct WaitingForApproval;

impl State for WaitingForApproval {
    fn approve(&self) -> Box<dyn State> {
        Box::new(Approved)
    }
    fn request_review(&self) -> Box<dyn State> {
        Box::new(Self)
    }
}

pub struct Approved;

impl State for Approved {
    fn show_content<'a>(&self, post: &'a Post) -> &'a str {
        post.content
    }
    fn approve(&self) -> Box<dyn State> {
        Box::new(Self)
    }
    fn request_review(&self) -> Box<dyn State> {
        Box::new(Self)
    }
}

pub struct Post<'a> {
    content: &'a str,
    state_type: Option<Box<dyn State>>,
}

impl<'a> Default for Post<'a> {
    fn default() -> Self {
        Self {
            content: "",
            state_type: Some(Box::new(Draft)),
        }
    }
}

impl<'a> Post<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_content(&'a self) -> &'a str {
        self.state_type.as_ref().unwrap().show_content(self)
    }

    pub fn add_text(&mut self, new_text: &'a str) {
        self.content = new_text;
    }

    pub fn approve(&mut self) {
        self.state_type = Some(self.state_type.as_ref().unwrap().approve());
    }
    pub fn request_review(&mut self) {
        self.state_type = Some(self.state_type.as_ref().unwrap().request_review());
    }
}

// State trait.
pub trait State {
    fn output(&self);
}

// A & B are values which will decide behaviour.
struct A;
impl State for A {
    fn output(&self) {
        println!("Inside A");
    }
}

pub struct B;

impl State for B {
    fn output(&self) {
        println!("Inside B");
    }
}

pub struct Target(Box<dyn State>);

impl Default for Target {
    fn default() -> Self {
        Self(Box::new(A))
    }
}

impl Target {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change_state(&mut self, new_state: Box<dyn State>) {
        self.0 = new_state;
    }

    pub fn output(&self) {
        self.0.output();
    }
}

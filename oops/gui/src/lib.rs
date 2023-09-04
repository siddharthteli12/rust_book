pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // dyn is used with trait object.
    // Here, component is not a concerte type.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
}

impl Draw for Button {
    fn draw(&self) {
        // My custom impl.
    }
}

pub struct Human {
    pub height: i32,
    pub name: String,
}

impl Draw for Human {
    fn draw(&self) {
        // My custom impl.
    }
}

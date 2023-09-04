use gui::{Button, Human, Screen};

fn main() {
    let lists = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 5,
            }),
            Box::new(Human {
                height: 5,
                name: String::from("Siddharth"),
            }),
        ],
    };
    lists.run();
}

use rlt32_traits::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Select box ({:?},{:?}) {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let b = Button {
        width: 10,
        height: 9,
        label: String::from("btn"),
    };
    let sb = SelectBox {
        width: 7,
        height: 8,
        options: vec![String::from("op1"), String::from("op1")],
    };
    let s = Screen {
        components: vec![Box::new(b), Box::new(sb)],
    };
    s.run();
}

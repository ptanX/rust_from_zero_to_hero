pub trait Animal {
    fn make_noise(&self);

    fn eat(&self);

    fn move_mechanism(&self);
}

pub struct Dog;

impl Animal for Dog {
    fn make_noise(&self) {
        println!("bark: go, go")
    }

    fn eat(&self) {
        println!("eat shit")
    }

    fn move_mechanism(&self) {
        println!("walk or run")
    }
}

pub struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("meow, meow")
    }

    fn eat(&self) {
        println!("eat rice")
    }

    fn move_mechanism(&self) {
        println!("walk or run")
    }
}
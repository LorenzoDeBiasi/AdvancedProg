pub mod dyn_ex {
    trait MyTrait {
        fn greet(&self);
    }

    impl MyTrait for String {
        fn greet(&self) {
            println!("Im string: {}", self);
        }
    }
    impl MyTrait for i32 {
        fn greet(&self) {
            println!("Im i32: {}", self.to_string());
        }
    }
    pub fn example() {
        let mut vec: Vec<Box<dyn MyTrait>> = Vec::new();
        vec.push(Box::new(String::from("Nigger")));
        vec.push(Box::new(21));
        vec.push(Box::new(23));
        vec.push(Box::new(33));
        vec.push(Box::new(String::from("BabboGay")));
        for elem in vec.iter(){
            elem.greet();
        }
    }
}
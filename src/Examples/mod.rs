pub mod dyn_ex {
    trait MyTrait {
        fn greet(&self);
    }
    impl MyTrait for String {
        fn greet(&self) { println!("Im string: {}", self); }
    }
    impl MyTrait for i32 {
        fn greet(&self) { println!("Im i32: {}", self.to_string()); }
    }
    fn get_filled_dyn_vec() -> Vec<Box<dyn MyTrait>>{
        let mut vec: Vec<Box<dyn MyTrait>> = Vec::new();
        vec.push(Box::new(String::from("Nigger")));
        vec.push(Box::new(21));
        vec.push(Box::new(23));
        vec.push(Box::new(33));
        vec.push(Box::new(String::from("BabboGay")));
        return vec;
    }
    pub fn example() {
        let vec = get_filled_dyn_vec();
        vec.iter().for_each(|x| x.greet());
    }
}

pub mod boxes{
    use std::ops::Deref;

    enum MyList<T>{
        Node(T, Box<MyList<T>>),
        Nil
    }

    struct Dereffirst{
        name: String,
        surname: String
    }
    struct Derefsecond{
        name: String,
        surname: String
    }
    enum Dereferable{
        Dereffirst(Dereffirst),
        Derefsecond(Derefsecond)
    }
    impl Deref for Dereferable{
        type Target = String;
        fn deref(&self) -> &Self::Target {
            return match self {
                Dereferable::Dereffirst(d) => { &d.name }
                Dereferable::Derefsecond(d) => { &d.surname }
            }
        }
    }
    fn derefering(){
        let boxedfirst: Box<Dereferable> = Box::new(Dereferable::Dereffirst(Dereffirst {
            name: "Lorenzo".to_string(),
            surname: "Rossi".to_string(),
        }));

        println!("{}\n", **boxedfirst);
    }

}
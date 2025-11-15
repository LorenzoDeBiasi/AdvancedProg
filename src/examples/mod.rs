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

pub mod sheet5{
    pub mod ex1{
        use std::ops::Deref;

        pub trait Printable{ fn print(&self); }
        impl Printable for i32{ fn print(&self) { println!("{}", self); } }
        impl Printable for String{ fn print(&self) { println!("{}", self); } }
        impl<T: Printable> Printable for Vec<T>{ fn print(&self) { self.iter().for_each(|x|x.print()); } }
        fn print<T>(x: T) where T: Printable { x.print(); }
        fn dyn_print<T>(b: Box<dyn Printable> ) where T: Printable { b.print() }
        pub fn example(){

        }

    }
    pub mod ex2{
        #[derive(Debug)]
        enum Category{
            Fiction,
            NonFiction,
            Science,
            History,
            Fantasy,
        }
        #[derive(Debug)]
        struct Book<'a>{ title: &'a str, cat: Category }
        #[derive(Debug)]
        struct Library<'a>{ bookcases: [Vec<Book<'a>>; 10] }

        impl Default for Library<'_>{
            fn default() -> Self {
                todo!()
            }
        }
    }
}

pub mod boxes{
    use std::fmt::Display;
    use std::ops::Deref;
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
    impl Display for Dereferable{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self{
                Dereferable::Dereffirst(d) => { write!(f, "{}", d.name) }
                Dereferable::Derefsecond(d) => { write!(f, "{}", d.surname) }
            }
        }
    }
    pub fn example() {
        let boxedfirst: Box<Dereferable> = Box::new(Dereferable::Dereffirst(Dereffirst {
            name: "Lorenzo".to_string(),
            surname: "Rossi".to_string(),
        }));
        let boxedsecond: Box<Dereferable> = Box::new(Dereferable::Derefsecond(Derefsecond {
            name: "Negro".to_string(),
            surname: "Amaro".to_string(),
        }));
        println!("{}\n{}\n", **boxedfirst, **boxedsecond);
        //qui l'auto deref coersion avviene al layer della String
        //Box<Dereferable> --deref--> Dereferable --deref--> String --implicit deref coercion--> &str
        println!("{}\n{}\n", boxedfirst, boxedsecond);
        //qui sfruttiamo il trait Display e la coersion di Box
        //Box<Dereferable> --Display--> Dereferable --Display--> String --implicit \\\deref coersion--> str

        //MORALE: println! non prova a fare coersion finché vuole, vuole solo qualcosa che implementa Display, quindi
        //o dereferenziamo noi fino a un (impl Display) oppure pssiamo una Box<T> che implementa Display SE T impl Display,
        //Box quindi come implementazione di Display passa direttamente T, la cui impl Display verrà effettivamente usata.
    }

}
pub mod arc{
    use std::thread;
    use std::sync::Arc;
    pub fn example(){
        let x = Arc::new("same data");
        println!("{}", x);
        for _ in 0..10{
            let x = Arc::clone(&x);
            thread::spawn(move || {
                println!("{:?} {:?}", x, thread::current().id());
            });
        }
    }
}
pub mod refcell{
    use std::cell::{RefCell, RefMut};

    trait ReadOnlyTrait{ fn some_func(&self); }

    struct MyStruct{ s: String }
    struct MyNewStruct{ s: RefCell<String> }

    impl ReadOnlyTrait for MyStruct {
        fn some_func(&self) {
            //does not work cause self is immutable
            //self.s.push('A');
            panic!("DOES NOT WORK FOR COMPILE TIME BORROW RULES");
        }
    }
    impl ReadOnlyTrait for MyNewStruct{
        fn some_func(&self) {
            let mut mut_borrow = self.s.borrow_mut();
            mut_borrow.push_str(" VIVA LA FIGA");

            //Senza questo panicheremmo, perché violeremmo le borrow rules:
            //usiamo una immutable borrow mentre mut_borrow è attiva
            std::mem::drop(mut_borrow);

            let borrow = self.s.borrow();
            println!("{}", borrow);
        }
    }

    pub fn example(){
        let x: MyNewStruct = MyNewStruct{s: RefCell::new(String::from("Silvio disse:"))};
        println!("{}", x.s.borrow());
        x.some_func();
    }
}
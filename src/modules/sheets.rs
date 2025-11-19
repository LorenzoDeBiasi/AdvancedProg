pub mod sheet5{
    #[allow(dead_code)]
    pub mod ex1{
        pub trait Printable{ fn print(&self); }

        impl Printable for i32{ fn print(&self) { println!("{}", self); } }
        impl Printable for String{ fn print(&self) { println!("{}", self); } }
        impl<T: Printable> Printable for Vec<T>{ fn print(&self) { self.iter().for_each(|x|x.print()); } }

        fn print<T>(x: T) where T: Printable { x.print(); }
        fn dyn_print<T>(b: Box<dyn Printable> ) where T: Printable { b.print() }

        pub fn example(){
            // Valori semplici
            let num = 42;
            let text = String::from("Hello");

            // Vettore misto
            let numbers = vec![1, 2, 3];
            let strings = vec![String::from("A"), String::from("B")];

            // Vettore di vettori
            let nested = vec![vec![1, 2], vec![3, 4]];

            // Stampa con funzione generica
            print(num);         // stampa: 42
            print(text);        // stampa: Hello
            print(numbers);     // stampa: 1\n2\n3
            print(strings);     // stampa: A\nB
            print(nested);      // stampa: 1\n2\n3\n4

            // Stampa con trait object
            let boxed: Box<dyn Printable> = Box::new(String::from("Boxed trait object"));
            dyn_print::<String>(boxed); // stampa: Boxed trait object
        }
    }

    #[allow(dead_code)]
    pub mod ex2{
        trait Populatable{ fn populate(&mut self); }

        #[derive(Default)] #[derive(Debug)]
        enum Category{
            Fiction,
            NonFiction,
            Science,
            History,
            #[default]
            Fantasy,
        }

        #[derive(Debug)]
        struct Book<'a>{ title: &'a str, cat: Category }

        #[derive(Debug)] #[derive(Default)]
        struct Library<'a>{ bookcases: [Vec<Book<'a>>; 10] }

        impl Default for Book<'_>{
            fn default() -> Self { Book{title: "Women Rights", cat: Category::Fantasy } }
        }
        impl Book<'_>{
            fn default_with_cat(category: Category) -> Self {
                let book = Book::default();
                Book{ cat: category, ..book}
            }
        }

        impl Populatable for Library<'_>{
            fn populate(&mut self){
                for bookcase in self.bookcases.iter_mut(){
                    let mut books = vec![Book::default(), Book::default(), Book::default()];
                    bookcase.append(&mut books);
                }
            }
        }

        pub fn example() {
            // Creiamo una libreria vuota con Default
            let mut library = Library::default();
            println!("{:#?}", library);

            // Popoliamo la libreria con 3 libri per ogni bookcase
            library.populate();

            // Stampiamo la libreria per vedere il risultato
            println!("{:#?}", library);

            // Dimostriamo anche l'uso di default_with_cat
            let special_book = Book::default_with_cat(Category::Science);
            println!("Libro speciale: {:?}", special_book);

            // Possiamo aggiungerlo manualmente al primo bookcase
            library.bookcases[0].push(special_book);

            println!("Dopo aver aggiunto il libro speciale:");
            println!("{:#?}", library.bookcases[0]);
        }
    }

    #[allow(dead_code)]
    pub mod ex3{
        use std::fmt::{Debug, Display};

        fn restricted<T, U>(t1: T, t2: T, u: U) -> T
        where T: Debug + PartialOrd, U: Display {
            match t1 < t2{
                true => {
                    println!("minor: {:?}\nu: {}", t1, u);
                    t1
                }
                false => {
                    println!("minor: {:?}\nu: {}", t2, u);
                    t2
                }
            }
        }

        pub fn example(){
            let x = 5;
            let y = 8;
            let z = "ciao";
            restricted(x, y, z);
        }
    }

    #[allow(dead_code)]
    pub mod ex4{
        struct Tasks{ tasks: Vec<Task> }
        struct Task{
            name: String,
            property: i32,
            done: bool
        }

        impl Task{
            fn new(name: String, property: i32) -> Self { Task{name, property, done: false} }
        }
        impl Tasks{
            fn new() -> Self{ Tasks{ tasks: Vec::new() } }
            fn add(&mut self, task: Task) { self.tasks.push(task); }
        }
        impl Iterator for Tasks{
            type Item = Task;
            fn next(&mut self) -> Option<Self::Item> {
                //elimino i task completati
                self.tasks.retain(|item| !item.done);

                if self.tasks.is_empty() {
                    None
                } else {
                    //essendo next( ) dobbiamo fare la pop del prossimo, non l'ultimo
                    //Some(self.tasks.pop().unwrap())

                    //e il prossimo sarà sempre il primo dopo la retain( )
                    Some(self.tasks.remove(0))
                }
            }
        }
    }

    #[allow(dead_code)]
    pub mod ex5{
        struct Pair(i32, String);

        impl std::ops::Add<i32> for Pair{
            type Output = Pair;
            fn add(self, rhs: i32) -> Self::Output {
                Pair(self.0 + rhs, self.1)
            }
        }
        impl std::ops::Sub<i32> for Pair{
            type Output = Pair;
            fn sub(self, rhs: i32) -> Self::Output {
                Pair(self.0 - rhs, self.1)
            }
        }

        impl std::ops::Add<&str> for Pair{
            type Output = Pair;
            fn add(self, rhs: &str) -> Self::Output {
                Pair(self.0, self.1 + rhs)
            }
        }
        impl std::ops::Sub<&str> for Pair{
            type Output = Pair;
            fn sub(self, rhs: &str) -> Self::Output {
                Pair(self.0, self.1.replace(rhs, ""))
            }
        }

        impl std::ops::Add<Pair> for Pair{
            type Output = Pair;
            fn add(self, rhs: Pair) -> Self::Output {
                Pair(self.0 + rhs.0, self.1 + rhs.1.as_str())
            }
        }
        impl std::ops::Sub<Pair> for Pair{
            type Output = Pair;
            fn sub(self, rhs: Pair) -> Self::Output {
                Pair(self.0 - rhs.0, self.1.replace(rhs.1.as_str(), ""))
            }
        }

        impl std::ops::Mul<i32> for Pair{
            type Output = Pair;

            fn mul(self, rhs: i32) -> Self::Output {
                Pair(self.0.pow(rhs as u32), self.1.repeat(rhs as usize))
            }
        }
    }

    #[allow(dead_code)]
    pub mod ex6{
        struct Open;
        struct Closed;
        struct Stopped { reason: String }
        struct Gate<S> { state: S }

        impl Open{
            fn close(self) -> Result<Gate<Closed>, Gate<Stopped>>{
                let chance: i32 = 32;
                match chance {
                    0..75 => { Ok(Gate { state: Closed {} }) }
                    _ => { Err(Gate { state: Stopped { reason: String::from("te si sfigà") } }) }
                }
            }
        }
        impl Closed{
            fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
                let chance: i32 = 88;
                match chance {
                    0..75 => { Ok(Gate { state: Open {} }) }
                    _ => { Err(Gate { state: Stopped { reason: String::from("te si sfigà") } }) }
                }
            }
        }
        impl Stopped{
            fn open(self) -> Gate<Open> { Gate{ state: Open{} } }
            fn close(self) -> Gate<Closed> { Gate{ state: Closed{} } }
        }

        fn new_gate() -> Gate<Closed>{ Gate{ state: Closed{} } }

        pub fn example(){
            /*
            let gate = new_gate();
            let gate = gate.state.open();
            let gate = match gate{
                Result::Ok(opened) => {
                    println!("Opened, now it gets closed");
                    opened.state.close()
                }
                Result::Err(stopped) => {
                    println!("error: {}, re opening", stopped.state.reason);
                    stopped.state.open()
                }
            }
            */
            println!("Questo design pattern con uno stato come struct non ha alcun senso ed è inutilizzabile");
        }
    }

    #[allow(dead_code)]
    pub mod ex7{
        use std::cell::RefCell;
        use std::rc::Rc;

        trait Heatable{ fn cook(&mut self); }
        trait Friable{ fn cook(&mut self); }
        trait Heater{ fn heat(&self, food: Rc<RefCell<dyn Heatable>>); }
        trait Frier{ fn fry(&self, food: Rc<RefCell<dyn Friable>>); }
        trait Edible{ fn eat(&self); }

        struct Oven{}
        struct Pan{}

        impl Heater for Oven{
            fn heat(&self, food: Rc<RefCell<dyn Heatable>>) { food.borrow_mut().cook(); }
        }
        impl Heater for Pan{
            fn heat(&self, food: Rc<RefCell<dyn Heatable>>) { food.borrow_mut().cook(); }
        }
        impl Frier for Pan{
            fn fry(&self, food: Rc<RefCell<dyn Friable>>) { food.borrow_mut().cook(); }
        }

        enum CarrotState{ Raw, Cooked, Fried , Burnt }

        struct Pie{ ready: bool }
        struct Carrot{ state: CarrotState }

        impl Pie{ fn new() -> Pie { Pie{ ready: false } } }
        impl Carrot{ fn new() -> Carrot { Carrot{ state: CarrotState::Raw } } }

        impl Heatable for Pie{
            fn cook(&mut self) {
                match self.ready{
                    false => {
                        println!("Pie is ready");
                        self.ready = true
                    },
                    true => println!("You burnt the pie, u idiot")
                }
            }
        }
        impl Heatable for Carrot{
            fn cook(&mut self) {
                match self.state{
                    CarrotState::Raw => {
                        println!("Carrot cooked");
                        self.state = CarrotState::Cooked;
                    }
                    _ => {
                        println!("You burnt the carrot, u idiot");
                        self.state = CarrotState::Burnt;
                    }
                }
            }
        }
        impl Friable for Carrot{
            fn cook(&mut self) {
                match self.state{
                    CarrotState::Fried => {
                        println!("You burnt the carrot, u idiot");
                        self.state = CarrotState::Burnt;
                    }
                    _ => {
                        println!("Carrot fried");
                        self.state = CarrotState::Fried;
                    }
                }
            }
        }
        impl Edible for Pie{
            fn eat(&self) {
                match self.ready{
                    true => { println!("bona dioprete"); }
                    false => { println!("codio vutu coparme? la sa ancora de tera"); }
                }
            }
        }
        impl Edible for Carrot{
            fn eat(&self) {
                match self.state{
                    CarrotState::Cooked => { println!("bona dioprete"); }
                    CarrotState::Fried => { println!("croccaaaante"); }
                    CarrotState::Raw => { println!("setu orbo? le cruda diocanci"); }
                    CarrotState::Burnt => { println!("codio vutu coparme?"); }
                }
            }
        }

        pub fn example() {
            // Creo gli strumenti
            let oven = Oven {};
            let pan = Pan {};

            // Creo i cibi
            let pie = Rc::new(RefCell::new(Pie::new()));
            let carrot = Rc::new(RefCell::new(Carrot::new()));

            println!("\ncuciniamo: ");
            oven.heat(carrot.clone());
            pan.fry(carrot.clone());

            {
                println!("\nmangiamo: ");
                carrot.borrow().eat();
                pie.borrow().eat();
            }

            println!("\nRicuciniamo:");
            pan.heat(carrot.clone());
            oven.heat(pie.clone());

            {
                println!("\nRimangiamo: ");
                carrot.borrow().eat();
                pie.borrow().eat();
            }
        }

    }
}


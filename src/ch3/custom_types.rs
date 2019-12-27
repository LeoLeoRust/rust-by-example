pub mod structures {
    #[derive(Debug)]
    //The 'a defines a lifetime
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    struct Nil;

    //A tuple struct
    struct Pair(i32, f32);

    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    pub fn struc_test() {
        // Create struct with field init shorthand
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };

        // Print debug struct
        println!("{:?}", peter);

        // Instantiate a `Point`
        let point: Point = Point { x: 10.3, y: 0.4 };

        // Access the fields of the point
        println!("point coordinates: ({}, {})", point.x, point.y);

        // Make a new point by using struct update syntax to use the fields of our
        // other one
        let bottom_right = Point { x: 5.2, ..point };

        // `bottom_right.y` will be the same as `point.y` because we used that field
        // from `point`
        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

        // Destructure the point using a `let` binding
        let Point {
            x: top_edge,
            y: left_edge,
        } = point;

        let _rectangle = Rectangle {
            // struct instantiation is an expression too
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: bottom_right,
        };

        // Instantiate a unit struct
        let _nil = Nil;

        // Instantiate a tuple struct
        let pair = Pair(1, 0.1);

        // Access the fields of a tuple struct
        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        // Destructure a tuple struct
        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);
    }
}

pub mod enums {
    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded!"),
            WebEvent::PageUnload => println!("page unloaded!"),
            WebEvent::KeyPress(c) => println!("pressed: {}.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x: {}, y:{}", x, y);
            }
        }
    }

    pub fn test_enum() {
        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` creates an owned `String` from a string slice.
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);
    }

    pub enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    pub use List::*;

    impl List {
        // add code here
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
                Nil => format!("Nil"),
            }
        }
    }

    pub fn test_list() {
        let mut list = List::new();

        // Prepend some elements
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        // Show the final state of the list
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}

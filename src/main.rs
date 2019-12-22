pub mod ch1 {
    pub mod hello_world;
}

pub mod ch2 {
    pub mod primitives;
}

fn main() {
    // println!("Hello, world!");
    // ch1::hello_world::display();
    // ch1::hello_world::format();
    // ch2::primitives::literals();
    // ch2::primitives::tuples();
    ch2::primitives::array_slice();
}

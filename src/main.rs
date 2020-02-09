pub mod ch1 {
    pub mod hello_world;
}

pub mod ch2 {
    pub mod primitives;
}

pub mod ch3 {
    pub mod custom_types;
}

fn main() {
    // println!("Hello, world!");
    // ch1::hello_world::display();
    // ch1::hello_world::format();
    // ch2::primitives::literals();
    // ch2::primitives::tuples();
    // ch2::primitives::array_slice();
    ch3::custom_types::structures::struc_test();
}

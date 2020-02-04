#![allow(dead_code)]

pub mod ch1 {
    pub mod hello_world;
}

pub mod ch2 {
    pub mod primitives;
}

pub mod ch3 {
    pub mod custom_types;
}

pub mod ch4 {
    pub mod variable_bindings;
}

pub mod  ch5 {
    pub mod types;
}

pub mod ch6 {
    pub mod conversion;
}

pub mod ch8 {
    pub mod flow_of_control;
}

pub mod ch9 {
    pub mod functions;
}

pub mod ch14 {
    pub mod generics;
}

fn main() {
    // println!("Hello, world!");
    // ch1::hello_world::display();
    // ch1::hello_world::format();
    // ch2::primitives::literals();
    // ch2::primitives::tuples();
    // ch2::primitives::array_slice();
    // ch3::custom_types::enums::test_enum();
    // ch3::custom_types::enums::test_list();
    // ch4::variable_bindings::scope_shadowing();
    // ch5::types::test_casting();
    // ch6::conversion::test_conver();
    // ch8::flow_of_control::test_foc();
    // ch9::functions::test_funcs();
    ch14::generics::functions::test_fib();
}

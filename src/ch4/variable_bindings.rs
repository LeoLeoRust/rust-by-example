
pub fn test_variable() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    
    let copied_integer = an_integer;
    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("unit: {:?}", unit);

    let _unused_variable = 3u32;
    let _noisy_unused_variable = 4u32;
}

pub fn test_mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 2;
    println!("After mutation: {}", mutable_binding);
}

pub fn scope_shadowing() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 3;
        let long_lived_binding = 5_f32;
        println!("inner short: {}", short_lived_binding);
        println!("inner long: {}", long_lived_binding);
    }

    println!("outer long lived binding befor: {}", long_lived_binding);
    //This binding also *shadows* the previous binding
    let long_lived_binding = 'a';

    println!("outer long lived binding: {}", long_lived_binding);
}
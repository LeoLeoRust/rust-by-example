#![allow(overflowing_literals)]

pub fn test_casting() {
    let decimal = 65.4321_f32;   
    // let integer: u8 = decimal;
    //Explicit conversation
    let integer = decimal as u8;
    let charcter = integer as char;
    println!("Casting from {} to {} to {}", decimal, integer, charcter);

    println!("1000 as u16 is {}", 1000 as u16);
    println!("1000 as u8 is {}", 1000 as u8);
    println!("-1 as u8 is {}", -1_i8 as u8);

    println!("1000 mod 256: {}", 1000 % 256);

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!(" 128 as a u8 is : {}", 128 as u8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
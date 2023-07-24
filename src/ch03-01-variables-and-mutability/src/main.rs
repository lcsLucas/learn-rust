fn main() {
    let mut x = 5;
    let y: i32 = 5;

    const Z: u32 = 100;

    println!("The value x is: {}, y is: {}, Z is: {}", x, y, Z);
    x = 6;
    println!("The new value x is: {x}, y is: {y}, Z is: {Z}");

    // # Sombreamento

    {
        let x = 500.5;

        println!("Shadowing x in scope is: {}", x);
    }

    println!("value of x is back to: {}", x);
}

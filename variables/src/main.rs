fn main() {
    println!("\nVariables and Mutability");
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}, after shadow", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The value of const MAX is: {}", MAX_POINTS);   

    println!("\nData Types");
    let string = "Rust rust rust";
    let boolean:bool = true; //Explicit type annotation
    let icon = '♥';
    let float = 4.5; 

    println!("A string is: {}" ,string);
    println!("A float is: {}", float);
    println!("An icon is: {}", icon);
    println!("Is Rust Programming Fun? {}", boolean);

    //Exits tuples, and arrays!
    
    another_function(2020);
}

fn another_function(day: i32){
    println!("\nAnother function, today is: {}", day);
}
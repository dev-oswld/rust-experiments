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

    //Exists tuples, and arrays!
    
    println!("\nFunctions and returns");
    another_function(18);
    let phrase = return_function("");
    println!("The string contains: {}", phrase);

    println!("\nControl flow");
    let condition = true;
    let number = if condition {
        5 //Here
    } else {
        6
    };

    println!("The value of number is: {}, if/else conditio", number);

    let mut another_number = 3;
    while another_number != 0 {
        println!("{}!", another_number);
        another_number -= 1;
    }
    println!("LIFTOFF!!!, while loop");

    //Exists loop, while, and for!
    //While VS for, the range is important
}

fn another_function(day: i32){
    println!("Another function, today is April {} ", day);
}

fn return_function(_x: &str) -> &str{
    return "Yep Yep";
}
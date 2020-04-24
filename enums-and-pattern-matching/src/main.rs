mod extra;

enum Profile {
    Age(i32)
    //Name(String),
}

fn main() {
    //let full_name = Profile::Name(String::from("Henry Ford"));
    let age_one = Profile::Age(83);

    match age_one {
        Profile::Age(value) => {
            if value >= 83 {
                println!("{} years", value);
            }
            else {
                println!("...");
            }
        }
    }

    /*match full_name {
        Profile::Name(value) => {
             println!("He is: {}", value);
        }
    }*/

    //Calling another rust file
    extra::hello();
}
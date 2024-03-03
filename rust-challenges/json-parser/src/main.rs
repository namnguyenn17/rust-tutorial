fn main() {
    println!("Hello, world!");

    let input = "Hello, world!";

    let res = convert_u32(input);

    match res {
        Some(x) => println!("Converted to u32: {}", x),
        None => println!("Failed to convert to u32"),
    }

    let peter = Peter {
        name: "Peter".to_string(),
        age: 20,
        height: "180".to_string(),
        weight: "70".to_string(),
    };

    println!("Peter's height: {}", peter.height());

    let foo = 5;
    print!("foo: {}", foo);
}

// Trait
pub trait People {
    fn height(&self) -> &String;
    fn weight(&self) -> &String;
}

struct Peter {
    name: String,
    age: u32,
    height: String,
    weight: String,
}

impl People for Peter {
    fn height(&self) -> &String {
        &self.height
    }

    fn weight(&self) -> &String {
        &self.weight
    }
}

fn convert_u32(input: &str) -> Option<u32> {
    let res = input.parse::<u32>().ok();
    res
}

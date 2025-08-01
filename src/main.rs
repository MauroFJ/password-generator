use std::env;
use rand::prelude::IteratorRandom;

#[derive(Debug)]
struct Validate{
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}

impl Validate {
    fn verify_true(&self) -> String {
        let mut chars = String::new();

        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let numbers = "0123456789";
        let symbols = "@#$&*-";

        if self.uppercase {
            chars.push_str(uppercase)
        }
        if self.lowercase {
            chars.push_str(lowercase)
        }
        if self.numbers {
            chars.push_str(numbers)
        }
        if self.symbols {
            chars.push_str(symbols)
        }

        chars
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let size: u8 = match args.get(1) {
        Some(arg) => match arg.parse::<u8>(){
            Ok(n) => n,
            Err(_) => panic!("Erro: Não transformado em Numero"),
        },
        None => {
            panic!("error: Passe um argumento.");
        }
    };

    let validate_args = Validate {
        uppercase: args.contains(&"--uppercase".to_string()),
        lowercase: args.contains(&"--lowercase".to_string()),
        numbers: args.contains(&"--numbers".to_string()),
        symbols: args.contains(&"--symbols".to_string()),
    };

    let result = validate_args.verify_true();
    let mut rng = rand::thread_rng();
    let mut random = String::new();

    if validate_args.symbols {
        random.push(result.chars().filter(|&c| "@#$&".contains(c)).choose(&mut rng).unwrap())
    }
    if validate_args.uppercase {
        random.push(result.chars().filter(|&c| c.is_uppercase()).choose(&mut rng).unwrap())
    }
     if validate_args.lowercase {
        random.push(result.chars().filter(|&c| c.is_lowercase()).choose(&mut rng).unwrap())
    }
     if validate_args.numbers {
        random.push(result.chars().filter(|&c| c.is_digit(10)).choose(&mut rng).unwrap())
    }

    while random.len() < size.into() {
        let random_char = result.chars().choose(&mut rng).unwrap();

        random.push(random_char);
    }

    println!("A senha gerada foi: {random}");
}
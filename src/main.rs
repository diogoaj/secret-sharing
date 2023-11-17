use std::{
    fs::File,
    io::{self, Read, Write},
};

use cli::{Cli, Commands};
use polynomial::{combine, split};

mod cli;
mod polynomial;
mod primitives;

fn main() {
    let cli = Cli::get_args();

    match &cli.command {
        Commands::Split(args) => {
            let mut f = File::open(&args.in_secret).unwrap();
            let mut secret = Vec::new();
            f.read_to_end(&mut secret).unwrap();

            let shares = split(secret, args.number, args.threshold);

            for share in shares {
                let y = hex::encode(&share[0..share.len() - 1]);
                let x = share[share.len() - 1];
                println!("{}-{}", x, y);
            }
        }
        Commands::Combine(args) => {
            let mut user_input = String::new();
            let mut shares: Vec<Vec<u8>> = Vec::new();

            println!("Enter 3 shares separated by newlines:");
            for i in 0..args.threshold {
                user_input.clear();

                print!("Share [{}/{}] ", i + 1, args.threshold);
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");

                let parts: Vec<&str> = user_input.split("-").collect();

                let x = parts[0].parse::<u8>().unwrap();
                let mut y_vec: Vec<u8> = hex::decode(parts[1].strip_suffix("\n").unwrap()).unwrap();
                y_vec.push(x);

                shares.push(y_vec);
            }

            let secret = combine(&shares);

            println!("Resulting secret: {}", String::from_utf8(secret).unwrap());
        }
    };
}

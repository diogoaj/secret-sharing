use std::io::{self, BufRead};

use cli::{Cli, Commands};
use polynomial::{combine, split};

mod cli;
mod polynomial;
mod primitives;

fn read_from_stdin() -> String {
    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    return input.strip_suffix("\n").unwrap().to_owned();
}

fn main() {
    let cli = Cli::get_args();

    match &cli.command {
        Commands::Split(args) => {
            let secret = read_from_stdin().as_bytes().to_vec();
            let shares = split(secret, args.number, args.threshold);

            for share in shares {
                let y = hex::encode(&share[0..share.len() - 1]);
                let x = share[share.len() - 1];
                println!("{}-{}", x, y);
            }
        }
        Commands::Combine(args) => {
            let mut shares: Vec<Vec<u8>> = Vec::new();
            let input = read_from_stdin();
            let lines: Vec<&str> = input.split("\n").collect();

            if lines.len() == args.threshold.into() {
                for line in lines {
                    let parts: Vec<&str> = line.split("-").collect();
                    let x = parts[0].parse::<u8>().unwrap();
                    let mut y_vec: Vec<u8> = hex::decode(parts[1]).unwrap();
                    y_vec.push(x);

                    shares.push(y_vec);
                }
            }

            let secret = combine(&shares);

            println!("Resulting secret: {}", String::from_utf8(secret).unwrap());
        }
    };
}

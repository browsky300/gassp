use clap::Parser;

#[derive(Parser)]
struct Cli {

    /// Password length
    #[arg(short, long, default_value_t = 12)]
    length: u8,

    /// Use numbers
    #[arg(short, long)]
    numbers: bool,

    /// Use special characters
    #[arg(short, long)]
    characters: bool,

    /// Don't use similar characters
    #[arg(short, long)]
    similar: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut chars = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    if cli.numbers {
        chars.push('1');
        chars.push('2');
        chars.push('3');
        chars.push('4');
        chars.push('5');
        chars.push('6');
        chars.push('7');
        chars.push('8');
        chars.push('9');
        if !cli.similar {
            chars.push('0');
        }
    }
    if cli.characters {
        chars.push('!');
        chars.push('@');
        chars.push('#');
        chars.push('$');
        chars.push('%');
        chars.push('&');
        chars.push('*');
        chars.push('-');
        chars.push('+');
        chars.push('=');
    }
    if !cli.similar {
        chars.push('I');
        chars.push('O');
        chars.push('l');
    }

    let mut password = String::from("");
    for _n in 0..cli.length {
        password = password + &chars[fastrand::usize(..chars.len())].to_string();
    }
    println!("{}", password);
}

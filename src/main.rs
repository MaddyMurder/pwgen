use clap::{Parser, Subcommand, Args};

use arboard::Clipboard;
use rand::seq::IndexedRandom;

#[derive(Parser)]
#[command(
    author,
    version,
    override_usage = "pwgen.exe <COMMAND> [ARGS]",
    subcommand_required(true),
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a password consisting of random characters.
    Password(PasswordArgs),
    /// Generate a username consisting of two words and some numbers.
    Username(UsernameArgs),
}

#[derive(Args)]
struct PasswordArgs {
    /// Amount of characters.
    /// 16 by default
    #[arg(short = 'l', long = "length")]
    length: Option<u32>,
    
    /// All the character sets used in the password.
    /// By default this is all of them.
    /// Example usage: "--char-set lower,upper,digits".
    #[arg(short = 'c', long = "char-set", value_enum, value_delimiter = ',')]
    character_sets: Option<Vec<CharSet>>,
    
    /// Exclude characters from the character sets.
    /// /// Example usage: "--exclude abc!@#".
    #[arg(short = 'e', long = "exclude")]
    excluded_chars: Option<String>,
    
    
    /// Disables copying the final password to clipboard.
    #[arg(short = 'n', long = "no-copy")]
    copy_disabled: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum CharSet {
    Lower,
    Upper,
    Digits,
    Symbol,
}

fn get_char_set (sets: &CharSet) -> Vec<char> {
    match sets {
        CharSet::Lower => {
            LOWER.to_vec()
        }
        CharSet::Upper => {
            UPPER.to_vec()
        }
        CharSet::Digits => {
            DIGITS.to_vec()
        }
        CharSet::Symbol => {
            SYMBOL.to_vec()
        }
    }
}

const LOWER: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];
const UPPER: &'static [char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];
const DIGITS: &'static [char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];
const SYMBOL: &'static [char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']',
    '{', '}', ':', ';', '.', ',', '?', '~'
];

#[derive(Args)]
struct UsernameArgs {
    /// Amount of numbers after the username
    /// 4 by default
    #[arg(short = 'n', long = "numbers")]
    numbers: Option<u32>,
    
    /// Character in between the two username words
    #[arg(short = 'c', long = "word-char")]
    word_char: Option<char>,
    
    /// Disables copying the final password to clipboard
    #[arg(short = 'N', long = "no-copy")]
    copy_disabled: bool,
}

const OBJECT_LIST_RAW: &'static str = include_str!("../data/object.txt");
const ADJECTIVE_LIST_RAW: &'static str = include_str!("../data/adjective.txt");

fn get_word_list(list: &'static str) -> Vec<&'static str> {
    list
        .lines()
        .collect()
}

fn main() {
    let cli = Cli::parse();
    
    let mut copy_to_clipboard = true;
    let mut final_string: String = "".into();
    let mut rng = rand::rng();
    
    match &cli.command {
        
        Commands::Password( args ) => {
            if args.copy_disabled {
                copy_to_clipboard = false;
            }
            
            
            let used_character_sets: &Vec<CharSet> = if let Some(selected_character_sets) = &args.character_sets {
                &selected_character_sets
            } else {
                &vec![CharSet::Lower, CharSet::Upper, CharSet::Digits, CharSet::Symbol] // Use default.
            };
            
            let mut all_characters: Vec<char> = Vec::new();
            for character_set in used_character_sets {
                all_characters.extend(get_char_set(character_set));
            }
            
            if let Some(excluded_chars) = &args.excluded_chars {
                for excluded_char in excluded_chars.chars().collect::<Vec<char>>() {
                    all_characters.retain(|&c| c != excluded_char);
                }
            }
            
            if all_characters.is_empty() {
                println!("No characters are allowed! Try to add more character sets or exclude less characters.");
                return;
            }
            
            let mut password_length = 16;
            if let Some(new_length) = args.length {
                if new_length > 16384 {
                    println!("Password too long! Cannot be longer than 16384.");
                    return;
                }
                password_length = new_length;
            }
            
            for _ in 0..password_length {
                let random_char = all_characters.choose(&mut rng).unwrap();
                final_string.push(*random_char);
            }
        }
        
        Commands::Username( args ) => {
            if args.copy_disabled {
                copy_to_clipboard = false;
            }
            
            
        }
    }
    
    println!("{}", final_string);
    
    if copy_to_clipboard {
        if let Ok(mut clipboard) = Clipboard::new() {
            let clipboard_success = clipboard.set_text(final_string);
            if clipboard_success.is_err() {
                println!("Unable to copy to clipboard.");
            }
        } else {
            println!("Unable to copy to clipboard because it is not supported by OS.");
        }
    }
}

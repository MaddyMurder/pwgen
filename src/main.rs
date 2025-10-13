use clap::{Parser, Subcommand, Args};
use arboard::Clipboard;
use rand::seq::IndexedRandom;
use rand::Rng;
use once_cell::sync::Lazy;

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
    /// Defaults to 16.
    #[arg(short = 'l', long = "length")]
    length: Option<u32>,
    
    /// All the character sets used in the password.
    /// By default this is all of them.
    /// Example usage: "--char-set lower,upper,digits".
    #[arg(short = 'c', long = "char-set", value_enum, value_delimiter = ',')]
    character_sets: Option<Vec<CharSet>>,
    
    /// Exclude characters from the character sets.
    /// Example usage: "--exclude abc!@#".
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

fn get_char_set (sets: &CharSet) -> &'static str {
    match sets {
        CharSet::Lower => LOWER,
        CharSet::Upper => UPPER,
        CharSet::Digits => DIGITS,
        CharSet::Symbol => SYMBOL,
    }
}

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOL: &str = "!@#$%^&*()-_=+[]{}:;.,?~";

#[derive(Args)]
struct UsernameArgs {
    /// Amount of numbers after the username.
    /// Defaults to 2.
    #[arg(short = 'N', long = "numbers")]
    numbers: Option<u32>,
    
    /// Character in between the words and numbers.
    /// Don't specify to omit it.
    #[arg(short = 'c', long = "word-char")]
    word_char: Option<char>,
    
    /// Disables copying the final password to clipboard.
    #[arg(short = 'n', long = "no-copy")]
    copy_disabled: bool,
}

const ADJECTIVE_LIST_RAW: &str = include_str!("../data/adjective.txt");
const OBJECT_LIST_RAW: &str = include_str!("../data/object.txt");

static ADJECTIVE_LIST: Lazy<Vec<&'static str>> = Lazy::new(|| {
    ADJECTIVE_LIST_RAW.lines().collect()
});
static OBJECT_LIST: Lazy<Vec<&'static str>> = Lazy::new(|| {
    OBJECT_LIST_RAW.lines().collect()
});

fn main() {
    let cli = Cli::parse();
    
    let mut copy_to_clipboard = true;
    let mut final_output: String = "".into();
    let mut rng = rand::rng();
    
    match &cli.command {
        
        Commands::Password( args ) => {
            if args.copy_disabled {
                copy_to_clipboard = false;
            }
            
            let chosen_character_sets: &[CharSet] = if let Some(user_character_sets) = &args.character_sets {
                user_character_sets
            } else {
                &[CharSet::Lower, CharSet::Upper, CharSet::Digits, CharSet::Symbol] // Use default.
            };
            
            let mut all_characters: Vec<char> = Vec::new();
            for character_set in chosen_character_sets {
                all_characters.extend(get_char_set(character_set).chars());
            }
            
            if let Some(excluded_chars) = &args.excluded_chars {
                for excluded_char in excluded_chars.chars() {
                    all_characters.retain(|&c| c != excluded_char);
                }
            }
            
            if all_characters.is_empty() {
                println!("No characters are allowed! Try to add more character sets or exclude less characters.");
                return;
            }
            
            let mut password_length = 16;
            if let Some(new_length) = args.length {
                if new_length > 65536 {
                    println!("Password too long! Cannot be longer than 65536.");
                    return;
                }
                password_length = new_length;
            }
            
            for _ in 0..password_length {
                let random_char = all_characters.choose(&mut rng).unwrap();
                final_output.push(*random_char);
            }
        }
        
        Commands::Username( args ) => {
            if args.copy_disabled {
                copy_to_clipboard = false;
            }
            
            let first_random_word = *ADJECTIVE_LIST.choose(&mut rng).expect("Adjective word list empty. This is a build error!");
            let second_random_word = *OBJECT_LIST.choose(&mut rng).expect("Object word list empty. This is a build error!");
            
            let chosen_word_char = if let Some(user_word_char) = args.word_char {
                user_word_char.to_string()
            } else {
                String::new()
            };
            
            final_output = format!("{}{}{}", first_random_word, chosen_word_char, second_random_word);
            
            let chosen_number_amount = if let Some(user_number_amount) = args.numbers {
                if user_number_amount > 65536 {
                    println!("Too many numbers! Cannot be more than 65536.");
                    return;
                }
                user_number_amount
            } else {
                2 // Default number amount.
            };
            
            if chosen_number_amount > 0 {
                final_output.push_str(&chosen_word_char);
            }
            
            for _ in 0..chosen_number_amount {
                final_output.push_str(&rng.random_range(0..10).to_string());
            }
        }
    }
    
    println!("{}", final_output);
    
    if copy_to_clipboard {
        if let Ok(mut clipboard) = Clipboard::new() {
            let clipboard_success = clipboard.set_text(final_output);
            if clipboard_success.is_ok() {
                println!("Copied to clipboard.");
            } else {
                println!("Unable to copy to clipboard.");
            }
        } else {
            println!("Unable to copy to clipboard because it is not supported by OS.");
        }
    }
}

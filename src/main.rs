extern crate shell_words;
use clap::Parser;
use std::io::{self, BufRead};
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Menu appears at the bottom of the screen
    #[clap(short = 'b', long = "bottom")]
    bottom: bool,

    /// Grabs the keyboard before reading stdin if not reading from a tty.
    /// This is faster, but will lock up X until stdin reaches end-of-file.
    #[clap(short = 'f')]
    grab_keyboard: bool,

    /// Matches menu items case insensitively.
    #[clap(short = 'i', long = "case-insensitive")]
    case_insensitive: bool,

    /// Lists items vertically, with the given number of lines.
    #[clap(short = 'l', long = "lines")]
    lines: Option<usize>,

    /// Displayed on monitor number supplied. Monitor numbers start from 0.
    #[clap(short = 'm', long = "monitor")]
    monitor: Option<usize>,

    /// Defines the prompt to be displayed to the left of the input field.
    #[clap(short = 'p', long = "prompt")]
    prompt: Option<String>,

    /// Defines the font or font set used
    #[clap(long = "fn")]
    font: Option<String>,

    /// Defines the normal background color. #RGB, #RRGGBB, and X color names are supported
    #[clap(long = "nb")]
    nm_bg: Option<String>,

    /// Defines the normal foreground color.
    #[clap(long = "nf")]
    nm_fg: Option<String>,

    /// Defines the selected background color.
    #[clap(long = "sb")]
    sl_bg: Option<String>,

    /// Defines the selected foreground color.
    #[clap(long = "sf")]
    sl_fg: Option<String>,

    /// Embed into windowid.
    #[clap(short = 'w', long = "window-id")]
    windowid: Option<String>
}

fn main() {
    // Read arguments
    let args = Args::parse();

    // Get prmopt
    let prompt = args.prompt.unwrap_or("Choose an Option: ".to_string());
    let prompt = format!("\\\"{}\\\"", prompt);

    // Get choices
    let choices: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.expect("Could not read line from stdin"))
        .collect();
    let choices = format!("'(\\\"{}\\\")", choices.join("\\\" \\\""));


    // Form the command to interact with ivy
    let command = format!("emacsclient --eval \"(ivy-read {} {})\"", prompt, choices);


    // Split up our command into arguments to be passed to std::process::Command
    let argv = shell_words::split(&command)
        .expect("Failed to parse shell command");

    // Run our command and capture the output
    let output = Command::new(&argv[0])
        .args(&argv[1..])
        .output()
        .expect("Failed to run ivy-read");

    // Format and print the output
    let output = String::from_utf8_lossy(&output.stdout);
    let output = &output[1..output.len() - 2];
    print!("{}", output);
}

use std::iter;
use structopt::StructOpt;

use unicode_width::UnicodeWidthStr;

const MAX_LINE_WIDTH: usize = 40;
const OCTOPUS: &str = "       \\
        \\       ⌢⌢
         \\   ◜      ◝
          \\ (        ) 
           ◟( (@)(@) )
             )  ◟◞  (
            /,'))((`.\\
           (( ((  )) ))
           ))`\\ `)(´/((";

#[derive(StructOpt)]
struct Cli {
    text: String,
}

// TODO: write tests
// TODO: break into separate (testable) functions, e.g. `format_string`

fn main() {
    let args = Cli::from_args();
    let mut lines = Vec::new();
    let words = args.text.split(" ");
    let mut current_line = String::new();

    for (i, word) in words.enumerate() {
        let word_width = word.width();

        if word_width > MAX_LINE_WIDTH {
            // special case: if one word is >40 chars, break it up with hyphens
            if !current_line.is_empty() {
                current_line.push(' ');
            }

            for c in word.chars() {
                if current_line.width() < MAX_LINE_WIDTH - 1 {
                    current_line.push(c);
                } else {
                    current_line.push('-');
                    lines.push(current_line);
                    current_line = c.to_string();
                }
            }
        } else if current_line.width() + word_width > MAX_LINE_WIDTH {
            // starts a new line
            lines.push(current_line);
            current_line = String::from(word);
        } else {
            if i != 0 {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if current_line.width() > 0 {
        lines.push(current_line);
    }

    let max_line_length = lines
        .iter()
        .map(|line| line.as_str().width())
        .max()
        .unwrap_or(0);

    let padded_lines = lines
        .into_iter()
        .map(|mut line| {
            let spaces = iter::repeat(" ")
                .take(max_line_length - line.as_str().width())
                .collect::<String>();
            line.push_str(&spaces);
            line
        })
        .collect::<Vec<_>>();

    let horizontal_border = iter::repeat("-")
        .take(max_line_length + 2)
        .collect::<String>();

    println!(" {} ", horizontal_border);
    for line in &padded_lines {
        println!("< {} >", line);
    }
    println!(" {} ", horizontal_border);
    println!("{}", OCTOPUS);
}

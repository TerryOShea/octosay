use std::iter;
use structopt::StructOpt;

use unicode_width::UnicodeWidthStr;

#[derive(StructOpt)]
struct Cli {
    text: String,
}

// TODO: exclamation marks do not work
// TODO: emoji/>1 byte unicode characters do not work

fn main() {
    let args = Cli::from_args();

    let mut lines = Vec::new();

    let words = args.text.split(" ");
    let mut substring = String::new();

    for (i, word) in words.enumerate() {
        if word.len() > 40 {
            if substring.len() != 0 {
                substring.push_str(" ");
            }

            for c in word.chars() {
                if substring.len() < 39 {
                    substring.insert(substring.len(), c);
                } else {
                    substring.push_str("-");
                    lines.push(substring);
                    substring = c.to_string();
                }
            }
        } else if substring.len() + word.len() > 40 {
            // starts a new line
            lines.push(substring);
            substring = String::from(word);
        } else {
            // this is triggered for the first word
            if i != 0 {
                substring.push_str(" ");
            }

            substring.push_str(word);
        }
    }

    if substring.len() > 0 {
        lines.push(substring);
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

    let octopus = "       \\
        \\       ⌢⌢
         \\   ◜      ◝
          \\ (        ) 
           ◟( (@)(@) )
             )  ◟◞  (
            /,'))((`.\\
           (( ((  )) ))
           ))`\\ `)(´/((";

    println!(" {} ", horizontal_border);
    for line in &padded_lines {
        println!("< {} >", line);
    }
    println!(" {} ", horizontal_border);
    println!("{}", octopus);
}

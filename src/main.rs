use std::{iter, cmp};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    text: String,
}

fn main() {
    let args = Cli::from_args();

    let mut lines = Vec::new();

    // TODO: what if words > 40
    let words = args.text.split(" ");
    let mut substring = String::new();
    let mut max_line_length = 0;

    for (i, word) in words.enumerate() {

        if word.len() > 40 {

            for (i, c) in word.chars().enumerate(){
                println!("{} {}", i, c);
            }


            /* 
                1. iterate through word by character, keeping track of the substring length so far
                2. if we're on char 39, add a hyphen and start a new substring from the following letter
            */
           

           

        } else if substring.len() + word.len() > 40 {

            max_line_length = cmp::max(max_line_length, substring.len());
            
            // starts a new line 
            lines.push(substring);
            substring = String::from(word);

        } else {

            // this is triggered for the first word
            if i != 0{
                substring.push_str(" ");
            }
            
            substring.push_str(word);
        }
    }

    if substring.len() > 0 {
        max_line_length = cmp::max(max_line_length, substring.len());
        lines.push(substring);
    }

    let padded_lines = lines.into_iter().map(|mut line| {
        let spaces = iter::repeat(" ").take(max_line_length - line.len()).collect::<String>();
        line.push_str(&spaces);
        line
    }).collect::<Vec<_>>();


    let horizontal_border = iter::repeat("-").take(max_line_length + 2).collect::<String>();
        
    let octopus = "       \\
        \\     ,´\"\"`.
         \\   / _  _ \\
          \\  |(@)(@)|
             )  ◟◞  (
            /,'))((`.\\
           (( ((  )) ))
            `\\ `)(´ /´";

    println!(" {} ", horizontal_border);
    for line in &padded_lines {
        println!("< {} >", line);
    }
    println!(" {} ", horizontal_border);
    println!("{}", octopus);
}

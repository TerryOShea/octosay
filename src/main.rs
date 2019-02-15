use std::{iter, cmp};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    text: String,
}

fn main() {
    let args = Cli::from_args();

    let mut lines = Vec::new();

    // TODO: what if multiple whitespace
    // TODO: what if words > 40
    let words = args.text.split(" ");
    let mut substring = String::new();
    let mut max_line_length = 0;

    for word in words {
        // TODO: don't push space for last word
        if substring.len() + word.len() > 39 {
            //let spaces = iter::repeat(" ").take(40 - substring.len()).collect::<String>();
            //substring.push_str(&spaces);
            max_line_length = cmp::max(max_line_length, substring.len());
            lines.push(substring);
            substring = String::from(word);
            substring.push_str(" ");
        } else {
            substring.push_str(word);
            substring.push_str(" ");
        }
    }
    if substring.len() > 0 {
        //let spaces = iter::repeat(" ").take(40 - substring.len()).collect::<String>();
        //substring.push_str(&spaces);
        max_line_length = cmp::max(max_line_length, substring.len());
        lines.push(substring);
    }

    let padded_lines = lines.into_iter().map(|mut line| {
        let spaces = iter::repeat(" ").take(max_line_length - line.len()).collect::<String>();
        line.push_str(&spaces);
        line
    }).collect::<Vec<_>>();


    //for mut line in lines {
    //    if line.len() < max_line_length {
    //        let spaces = iter::repeat(" ").take(max_line_length - line.len()).collect::<String>();
    //        let padded_line
    //        let padded_line = line.push_str(&spaces);
    //        padded_lines.push(padded_line);
    //    }
    //}

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

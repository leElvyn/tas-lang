extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "tas.pest"]
pub struct CSVParser;

fn main() {
    let mut unparsed_file = fs::read_to_string("script.taspl").expect("cannot read file");

    if unparsed_file.chars().last() != Some('\n') {
        // Add a newline if it's missing. if file is empty, parse will fail, don't add a new line
        unparsed_file.push('\n');
    }

    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut output = String::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::full_input => {

                let mut input_type = String::new();
                let mut frame_number: u32 = 0;
                line.into_inner().for_each(|input| {
                    match input.as_rule() {
                        
                        Rule::input => {
                            input_type = format!("{}", input.as_str());
                        },
                        Rule::frame_argument => {
                            
                            frame_number = input.into_inner().next().unwrap().as_str()
                                .parse::<u32>().expect("unsuccessful parse");
                        }
                        _ => unreachable!(),
                    }
                });

                (0..frame_number).for_each(|i| {
                    output.push_str(&format!("{} {}\n", i, input_type));
                });
            },
                 
            Rule::EOI => (),
            _ => (),
        }
        output.push('\n');
    }
    println!("{}", output);
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pair};
use pest::RuleType;
use std::fs;
use std::io::prelude::*;
use std::time::{Instant};

#[derive(Parser)]
#[grammar = "tas.pest"]
pub struct TASParser;


fn write_to_file(result: &String) -> std::io::Result<()> {
    let mut file = fs::File::create("script1-0.txt")?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

fn process_loop(block: Pair<Rule>) -> String {
    let mut result = String::new();

    result
}

fn process_block(block: Pair<Rule>) {
    let block_type: RuleType;
    for block_part in block.into_inner() {
        // for every block part. This includes the block declaration, and the block body
        match block_part.as_rule() {
            Rule::block_declaration => {
                // for every block declaration.
                for block_declaration_type in block_part.into_inner() {
                    print!("{:?}", block_declaration_type);
                    
                }
            },
            _ => ()
        }

    }
}

fn main() {
    
    let mut unparsed_file = fs::read_to_string("script.taspl").expect("cannot read file");
    
    if unparsed_file.chars().last() != Some('\n') {
        // Add a newline if it's missing. if file is empty, parse will fail, don't add a new line
        unparsed_file.push('\n');
    }
    
    let now = Instant::now();
    
    let file = TASParser::parse(Rule::file, &unparsed_file)
    .expect("unsuccessful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    println!("Parsing success in {} seconds", now.elapsed().as_secs_f32());
    let now = Instant::now();
    
    let mut output = String::new();

    let mut frame_counter = 0;

    for line in file.into_inner() {
        // matching singular lines
        match line.as_rule() {
            Rule::line => {
                // for every instruction line

                let input_type: String;
                let frame_number: u32;
                // number of times to repeat the line


                let instruction = line.into_inner().next().unwrap();
                // for now, we only support one instruction per line
                let mut instruction_iter = instruction.into_inner();

                let input = instruction_iter.next().unwrap();
                        
                input_type = format!("{:?}", input.into_inner().next().unwrap().as_rule());
                        
                let frame_argument = instruction_iter.next().unwrap();
                            
                frame_number = frame_argument.into_inner().next().unwrap().as_str()
                    .parse::<u32>().expect("unsuccessful parse");
                        

                (0..frame_number).for_each(|_| {
                    frame_counter += 1;
                    output.push_str(&format!("{} {} 0;0 0;0\n", frame_counter, input_type));
                });
            },

            Rule::block => {
                // for every block. currentlu only loop blocks are supported
                let block_result = process_block(line);
            }
            Rule::EOI => (),
            _ => (),
        }
    }
    println!("Compilation success in {} seconds", now.elapsed().as_secs_f32());

    let _ = write_to_file(&output);

    


}
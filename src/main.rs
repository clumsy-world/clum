use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ClumParser;

fn main() {
    let code = "data |> process |> save |> !";
    
    match ClumParser::parse(Rule::program, code) {
        Ok(pairs) => {
            for pair in pairs {
                println!("{:#?}", pair);
            }
        }
        Err(e) => println!("{}", e),
    }
}
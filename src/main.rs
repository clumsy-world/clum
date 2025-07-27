use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ClumParser;

fn main() {
    let examples = vec![
        // "data |> process |> save |> !",
        // "hello |> !",
        // "user-data |> validate |> !",
        "100-yen |> validate |> !",
    ];
    
    for code in examples {
        println!("\n=== Parsing: {} ===", code);
        match ClumParser::parse(Rule::program, code) {
            Ok(pairs) => {
                for pair in pairs {
                    print_pair(&pair, 0);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn print_pair(pair: &pest::iterators::Pair<Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{}Rule: {:?}, Text: '{}'", indent_str, pair.as_rule(), pair.as_str());
    
    for inner in pair.clone().into_inner() {
        print_pair(&inner, indent + 1);
    }
}

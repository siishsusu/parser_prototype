use pest::{Parser, parses_to};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file, "-271.56,-15.5,666\n")?;
    println!("{:?}", got);
    Ok(())
}

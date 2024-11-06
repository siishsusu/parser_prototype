use anyhow::anyhow;
use pest::{Parser, RuleType};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {

    Ok(())
}

#[test]
fn basic_test() -> anyhow::Result< () > {
    let got = Grammar::parse(Rule::file, "-273.15,-15\n")?;
    println!("{:?}", got);

    Ok(())
}

#[test]
fn field_test() -> anyhow::Result< () > {

    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn record_test() -> anyhow::Result< () > {

    let pair = Grammar::parse(Rule::record, "-273.15,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15,99" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );

    let pair = Grammar::parse(Rule::record, "");
    assert!( pair.is_err() );

    Ok( () )
}

#[test]
fn file_test() -> anyhow::Result< () > {
    let pair = Grammar::parse(Rule::file, "-273.15,99\n15,-5\n")?
        .next()
        .ok_or_else( || anyhow!( "no pair" ) )?;

    assert_eq!( pair.as_str(), "-273.15,99\n15,-5\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 17 );

    let pair = Grammar::parse(Rule::file, "-273.15,99,someText\n");
    assert!(pair.is_err());

    Ok(())
}
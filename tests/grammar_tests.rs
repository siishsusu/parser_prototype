use pest::Parser;
use anyhow::anyhow;
use pest_01::*;

#[test]
fn field_test() -> anyhow::Result< () > {


    // Тестуємо простий числовий вхід
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    // Тест на неправильний вхід
    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());
    // Тест на порожній вхід
    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}


#[test]
fn record_test() -> anyhow::Result< () > {

    // Тестуємо правильний формат запису
    let pair = Grammar::parse(Rule::record, "-273.15,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15,99" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );

    // Неправильний запис без чисел
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

#[test]
fn file_test_single_record() -> anyhow::Result<()> {
    // Тестуємо файл з одним записом
    let pair = Grammar::parse(Rule::file, "-273.15,99\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-273.15,99\n");

    Ok(())
}

#[test]
fn file_test_multiple_records() -> anyhow::Result<()> {
    // Тестуємо файл з кількома записами
    let pair = Grammar::parse(Rule::file, "-273.15,99\n36.6,98.6\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-273.15,99\n36.6,98.6\n");

    Ok(())
}

#[test]
fn file_test_incorrect_format() {
    // Тестуємо некоректний формат, наприклад, без коми між числами
    let pair = Grammar::parse(Rule::file, "-273.15 99\n");
    assert!(pair.is_err());

    // Відсутність нового рядка між записами
    let pair = Grammar::parse(Rule::file, "-273.15,99-36.6,98.6");
    assert!(pair.is_err());
}
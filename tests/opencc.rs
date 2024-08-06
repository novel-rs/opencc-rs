use std::fs;

use opencc_rs::{Config, OpenCC};
use pretty_assertions::assert_eq;
use testresult::TestResult;

#[cfg(target_os = "windows")]
macro_rules! PATH_SEPARATOR {
    () => {
        r"\"
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! PATH_SEPARATOR {
    () => {
        r"/"
    };
}

macro_rules! TESTCASES_PREFIX {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            PATH_SEPARATOR!(),
            "opencc-sys",
            PATH_SEPARATOR!(),
            "OpenCC",
            PATH_SEPARATOR!(),
            "test",
            PATH_SEPARATOR!(),
            "testcases",
            PATH_SEPARATOR!()
        )
    };
}

#[test]
fn hk2s() -> TestResult {
    let opencc = OpenCC::new([Config::HK2S])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "hk2s.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "hk2s.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn hk2t() -> TestResult {
    let opencc = OpenCC::new([Config::HK2T])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "hk2t.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "hk2t.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn jp2t() -> TestResult {
    let opencc = OpenCC::new([Config::JP2T])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "jp2t.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "jp2t.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn s2hk() -> TestResult {
    let opencc = OpenCC::new([Config::S2HK])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2hk.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2hk.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn s2t() -> TestResult {
    let opencc = OpenCC::new([Config::S2T])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2t.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2t.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn s2tw() -> TestResult {
    let opencc = OpenCC::new([Config::S2TW])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2tw.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2tw.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn s2twp() -> TestResult {
    let opencc = OpenCC::new([Config::S2TWP])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2twp.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "s2twp.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn t2hk() -> TestResult {
    let opencc = OpenCC::new([Config::T2HK])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2hk.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2hk.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn t2jp() -> TestResult {
    let opencc = OpenCC::new([Config::T2JP])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2jp.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2jp.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn t2s() -> TestResult {
    let opencc = OpenCC::new([Config::T2S])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2s.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "t2s.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn tw2s() -> TestResult {
    let opencc = OpenCC::new([Config::TW2S])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2s.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2s.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn tw2sp() -> TestResult {
    let opencc = OpenCC::new([Config::TW2SP])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2sp.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2sp.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn tw2t() -> TestResult {
    let opencc = OpenCC::new([Config::TW2T])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2t.in"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2t.ans"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn t2tw() -> TestResult {
    let opencc = OpenCC::new([Config::T2TW])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2t.ans"))?;
    let output = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "tw2t.in"))?;

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

#[test]
fn jp2t2s() -> TestResult {
    let opencc = OpenCC::new([Config::JP2T, Config::T2S])?;

    let input = fs::read_to_string(concat!(TESTCASES_PREFIX!(), "jp2t.in"))?;
    let output = include_str!("jp2t2s.ans").trim();

    assert_eq!(opencc.convert(input)?, output);

    Ok(())
}

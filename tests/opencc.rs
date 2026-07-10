use std::collections::HashMap;
use std::fs;

use opencc_rs::{Config, OpenCC};
use pretty_assertions::assert_eq;
use serde::Deserialize;
use testresult::TestResult;

#[derive(Debug, Deserialize)]
struct TestCases {
    cases: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
struct TestCase {
    id: String,
    input: String,
    expected: HashMap<String, String>,
}

#[test]
fn all() -> TestResult {
    let test_cases = fs::read_to_string(format!(
        "{}/tests/testcases.json",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let test_cases: TestCases = serde_json::from_str(&test_cases)?;

    for case in test_cases.cases {
        for (config_name, expected) in &case.expected {
            let opencc = OpenCC::new([config_from_name(config_name)])?;
            let actual = opencc.convert(&case.input)?;

            assert_eq!(
                expected, &actual,
                "case `{}` failed for config `{}` with input {:?}",
                case.id, config_name, case.input
            );
        }
    }

    Ok(())
}

fn config_from_name(name: &str) -> Config {
    match name {
        "s2t" => Config::S2T,
        "t2s" => Config::T2S,
        "s2tw" => Config::S2TW,
        "tw2s" => Config::TW2S,
        "s2hk" => Config::S2HK,
        "hk2s" => Config::HK2S,
        "s2twp" => Config::S2TWP,
        "tw2sp" => Config::TW2SP,
        "t2tw" => Config::T2TW,
        "tw2t" => Config::TW2T,
        "t2hk" => Config::T2HK,
        "hk2t" => Config::HK2T,
        "s2hkp" => Config::S2HKP,
        "hk2sp" => Config::HK2SP,
        "t2jp" => Config::T2JP,
        "jp2t" => Config::JP2T,
        _ => unreachable!(),
    }
}

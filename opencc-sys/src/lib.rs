#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use lazy_static::lazy_static;

#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {
    () => {
        r"\"
    };
}

#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {
    () => {
        r"/"
    };
}

include!(concat!(env!("OUT_DIR"), PATH_SEPARATOR!(), "bindings.rs"));

macro_rules! JSON_PREFIX {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            PATH_SEPARATOR!(),
            "OpenCC",
            PATH_SEPARATOR!(),
            "data",
            PATH_SEPARATOR!(),
            "config",
            PATH_SEPARATOR!()
        )
    };
}

macro_rules! OCD2_PREFIX {
    () => {
        concat!(
            env!("OUT_DIR"),
            PATH_SEPARATOR!(),
            "build",
            PATH_SEPARATOR!(),
            "data",
            PATH_SEPARATOR!()
        )
    };
}

pub struct Data {
    pub filename: &'static str,
    pub content: &'static [u8],
}

macro_rules! new_json_data {
    ($filename:expr) => {
        Data {
            filename: $filename,
            content: include_bytes!(concat!(JSON_PREFIX!(), $filename)),
        }
    };
}

macro_rules! new_ocd2_data {
    ($filename:expr) => {
        Data {
            filename: $filename,
            content: include_bytes!(concat!(OCD2_PREFIX!(), $filename)),
        }
    };
}

lazy_static! {
    pub static ref HK2S_JSON: Data = new_json_data!("hk2s.json");
    pub static ref HK2T_JSON: Data = new_json_data!("hk2t.json");
    pub static ref JP2T_JSON: Data = new_json_data!("jp2t.json");
    pub static ref S2HK_JSON: Data = new_json_data!("s2hk.json");
    pub static ref S2T_JSON: Data = new_json_data!("s2t.json");
    pub static ref S2TW_JSON: Data = new_json_data!("s2tw.json");
    pub static ref S2TWP_JSON: Data = new_json_data!("s2twp.json");
    pub static ref T2HK_JSON: Data = new_json_data!("t2hk.json");
    pub static ref T2JP_JSON: Data = new_json_data!("t2jp.json");
    pub static ref T2S_JSON: Data = new_json_data!("t2s.json");
    pub static ref T2TW_JSON: Data = new_json_data!("t2tw.json");
    pub static ref TW2S_JSON: Data = new_json_data!("tw2s.json");
    pub static ref TW2SP_JSON: Data = new_json_data!("tw2sp.json");
    pub static ref TW2T_JSON: Data = new_json_data!("tw2t.json");
    pub static ref HKVARIANTS_OCD2: Data = new_ocd2_data!("HKVariants.ocd2");
    pub static ref HKVARIANTS_REV_OCD2: Data = new_ocd2_data!("HKVariantsRev.ocd2");
    pub static ref HKVARIANTS_REV_PHRASES_OCD2: Data = new_ocd2_data!("HKVariantsRevPhrases.ocd2");
    pub static ref JPSHINJITAI_CHARATERS_OCD2: Data = new_ocd2_data!("JPShinjitaiCharacters.ocd2");
    pub static ref JPSHINJITAI_PHRASES_OCD2: Data = new_ocd2_data!("JPShinjitaiPhrases.ocd2");
    pub static ref JPVARIANTS_OCD2: Data = new_ocd2_data!("JPVariants.ocd2");
    pub static ref JPVARIANTS_REV_OCD2: Data = new_ocd2_data!("JPVariantsRev.ocd2");
    pub static ref STCHARACTERS_OCD2: Data = new_ocd2_data!("STCharacters.ocd2");
    pub static ref STPHRASES_OCD2: Data = new_ocd2_data!("STPhrases.ocd2");
    pub static ref TSCHARACTERS_OCD2: Data = new_ocd2_data!("TSCharacters.ocd2");
    pub static ref TSPHRASES_OCD2: Data = new_ocd2_data!("TSPhrases.ocd2");
    pub static ref TWPHRASES_OCD2: Data = new_ocd2_data!("TWPhrases.ocd2");
    pub static ref TWPHRASES_REV_OCD2: Data = new_ocd2_data!("TWPhrasesRev.ocd2");
    pub static ref TWVARIANTS_OCD2: Data = new_ocd2_data!("TWVariants.ocd2");
    pub static ref TWVARIANTS_REV_OCD2: Data = new_ocd2_data!("TWVariantsRev.ocd2");
    pub static ref TWVARIANTS_REV_PHRASES_OCD2: Data = new_ocd2_data!("TWVariantsRevPhrases.ocd2");
}

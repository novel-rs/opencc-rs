#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate link_cplusplus;

use std::sync::LazyLock;

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::*;

macro_rules! JSON_PREFIX {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/OpenCC/data/config/",)
    };
}

macro_rules! OCD2_PREFIX {
    () => {
        "assets/"
    };
}

pub struct Data {
    pub file_name: &'static str,
    pub content: &'static [u8],
}

macro_rules! new_json_data {
    ($file_name:expr) => {
        LazyLock::new(|| Data {
            file_name: $file_name,
            content: include_bytes!(concat!(JSON_PREFIX!(), $file_name)),
        })
    };
}

macro_rules! new_ocd2_data {
    ($file_name:expr) => {
        LazyLock::new(|| Data {
            file_name: $file_name,
            content: include_bytes!(concat!(OCD2_PREFIX!(), $file_name)),
        })
    };
}

pub static HK2S_JSON: LazyLock<Data> = new_json_data!("hk2s.json");
pub static HK2SP_JSON: LazyLock<Data> = new_json_data!("hk2sp.json");
pub static HK2T_JSON: LazyLock<Data> = new_json_data!("hk2t.json");
pub static JP2T_JSON: LazyLock<Data> = new_json_data!("jp2t.json");
pub static S2HK_JSON: LazyLock<Data> = new_json_data!("s2hk.json");
pub static S2HKP_JSON: LazyLock<Data> = new_json_data!("s2hkp.json");
pub static S2T_JSON: LazyLock<Data> = new_json_data!("s2t.json");
pub static S2TW_JSON: LazyLock<Data> = new_json_data!("s2tw.json");
pub static S2TWP_JSON: LazyLock<Data> = new_json_data!("s2twp.json");
pub static T2HK_JSON: LazyLock<Data> = new_json_data!("t2hk.json");
pub static T2JP_JSON: LazyLock<Data> = new_json_data!("t2jp.json");
pub static T2S_JSON: LazyLock<Data> = new_json_data!("t2s.json");
pub static T2TW_JSON: LazyLock<Data> = new_json_data!("t2tw.json");
pub static TW2S_JSON: LazyLock<Data> = new_json_data!("tw2s.json");
pub static TW2SP_JSON: LazyLock<Data> = new_json_data!("tw2sp.json");
pub static TW2T_JSON: LazyLock<Data> = new_json_data!("tw2t.json");

pub static CJK_COMPATIBILITY_IDEOGRAPHS_OCD2: LazyLock<Data> =
    new_ocd2_data!("CJK_Compatibility_Ideographs.ocd2");
pub static HK_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("HKPhrases.ocd2");
pub static HK_PHRASES_REV_OCD2: LazyLock<Data> = new_ocd2_data!("HKPhrasesRev.ocd2");
pub static HK_VARIANTS_OCD2: LazyLock<Data> = new_ocd2_data!("HKVariants.ocd2");
pub static HK_VARIANTS_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("HKVariantsPhrases.ocd2");
pub static HK_VARIANTS_REV_OCD2: LazyLock<Data> = new_ocd2_data!("HKVariantsRev.ocd2");
pub static HK_VARIANTS_REV_PHRASES_OCD2: LazyLock<Data> =
    new_ocd2_data!("HKVariantsRevPhrases.ocd2");
pub static JP_SHINJITAI_CHARATERS_OCD2: LazyLock<Data> =
    new_ocd2_data!("JPShinjitaiCharacters.ocd2");
pub static JP_SHINJITAI_CHARACTERS_REV_OCD2: LazyLock<Data> =
    new_ocd2_data!("JPShinjitaiCharactersRev.ocd2");
pub static JP_SHINJITAI_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("JPShinjitaiPhrases.ocd2");
pub static ST_CHARACTERS_OCD2: LazyLock<Data> = new_ocd2_data!("STCharacters.ocd2");
pub static ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2: LazyLock<Data> =
    new_ocd2_data!("STPhrases_GeneratedFromRegionalPhrases.ocd2");
pub static ST_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("STPhrases.ocd2");
pub static TS_CHARACTERS_OCD2: LazyLock<Data> = new_ocd2_data!("TSCharacters.ocd2");
pub static TS_CHARACTERS_EXT_OCD2: LazyLock<Data> = new_ocd2_data!("TSCharactersExt.ocd2");
pub static TS_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("TSPhrases.ocd2");
pub static TW_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("TWPhrases.ocd2");
pub static TW_PHRASES_REV_OCD2: LazyLock<Data> = new_ocd2_data!("TWPhrasesRev.ocd2");
pub static TW_VARIANTS_OCD2: LazyLock<Data> = new_ocd2_data!("TWVariants.ocd2");
pub static TW_VARIANTS_PHRASES_OCD2: LazyLock<Data> = new_ocd2_data!("TWVariantsPhrases.ocd2");
pub static TW_VARIANTS_REV_OCD2: LazyLock<Data> = new_ocd2_data!("TWVariantsRev.ocd2");
pub static TW_VARIANTS_REV_PHRASES_OCD2: LazyLock<Data> =
    new_ocd2_data!("TWVariantsRevPhrases.ocd2");

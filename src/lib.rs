//! OpenCC bindings for Rust

use std::ffi::{CStr, CString, c_void};
use std::{fs, io};

use libc::uintptr_t;
use thiserror::Error;

/// OpenCC bindings for Rust
pub struct OpenCC {
    openccs: Vec<*mut c_void>,
}

impl OpenCC {
    /// Create a new OpenCC instance with the given configuration
    pub fn new<T>(configs: T) -> Result<OpenCC, Error>
    where
        T: AsRef<[Config]>,
    {
        let configs = configs.as_ref();
        assert!(!configs.is_empty());

        let mut openccs = Vec::new();

        for config in configs {
            let config_data = config.get_data();
            let dir = tempfile::tempdir()?;
            for item in &config_data {
                let file_path = dir.path().join(item.file_name);
                fs::write(file_path, item.content)?;
            }

            let config_file_path = dir.path().join(config_data[0].file_name);
            let config_file_path = CString::new(config_file_path.to_str().unwrap()).unwrap();

            let opencc = unsafe { opencc_sys::opencc_open(config_file_path.as_ptr()) };

            let ptr = opencc as uintptr_t;
            if ptr == uintptr_t::MAX {
                return Err(Error::Create);
            }

            openccs.push(opencc);
        }

        Ok(OpenCC { openccs })
    }

    /// Convert a string to another string
    pub fn convert<T>(&self, input: T) -> Result<String, Error>
    where
        T: AsRef<str>,
    {
        let mut length = input.as_ref().len();
        let input = CString::new(input.as_ref()).unwrap();
        let mut result_ptr = input.as_ptr().cast_mut();

        let mut free = Vec::new();

        for opencc in &self.openccs {
            result_ptr = unsafe { opencc_sys::opencc_convert_utf8(*opencc, result_ptr, length) };
            if result_ptr.is_null() {
                return Err(Error::Convert);
            }

            free.push(result_ptr);

            if self.openccs.len() > 1 {
                length = unsafe { libc::strlen(result_ptr) };
            }
        }

        let result_cstr = unsafe { CStr::from_ptr(result_ptr) };
        let result = unsafe { std::str::from_utf8_unchecked(result_cstr.to_bytes()).to_string() };

        for ptr in free {
            unsafe {
                opencc_sys::opencc_convert_utf8_free(ptr);
            }
        }

        Ok(result)
    }
}

impl Drop for OpenCC {
    fn drop(&mut self) {
        for opencc in &self.openccs {
            if !opencc.is_null() {
                unsafe {
                    opencc_sys::opencc_close(*opencc);
                }
            }
        }
    }
}

unsafe impl Send for OpenCC {}

unsafe impl Sync for OpenCC {}

/// Configurations
pub enum Config {
    /// Simplified Chinese to Traditional Chinese (OpenCC Standard)
    S2T,
    /// Traditional Chinese (OpenCC Standard) to Simplified Chinese
    T2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    S2TW,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese
    TW2S,
    /// Simplified Chinese to Traditional Chinese (Hong Kong variant)
    S2HK,
    /// Traditional Chinese (Hong Kong variant) to Simplified Chinese
    HK2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard, with Taiwan Phrases)
    S2TWP,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese (Mainland China Phrases)
    TW2SP,
    /// Traditional Chinese (OpenCC Standard) to Traditional Chinese (Taiwan Standard)
    T2TW,
    /// Traditional Chinese (Taiwan Standard) to Traditional Chinese (OpenCC Standard)
    TW2T,
    /// Traditional Chinese (OpenCC Standard) to Traditional Chinese (Hong Kong variant)
    T2HK,
    /// Traditional Chinese (Hong Kong variant) to Traditional Chinese (OpenCC Standard)
    HK2T,
    /// Simplified Chinese to Traditional Chinese (Hong Kong variant, with Hong Kong Phrases)
    S2HKP,
    /// Traditional Chinese (Hong Kong variant) to Simplified Chinese (Mainland China Phrases)
    HK2SP,
    /// Old Japanese Kanji (Kyūjitai) to New Japanese Kanji (Shinjitai)
    T2JP,
    /// New Japanese Kanji (Shinjitai) to Old Japanese Kanji (Kyūjitai)
    JP2T,
}

impl Config {
    fn get_data(&self) -> Vec<&opencc_sys::Data> {
        use opencc_sys::*;

        match self {
            Config::S2T => vec![
                &S2T_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &ST_PHRASES_OCD2,
                &ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2,
                &ST_CHARACTERS_OCD2,
            ],
            Config::T2S => vec![
                &T2S_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TS_PHRASES_OCD2,
                &TS_CHARACTERS_EXT_OCD2,
                &TS_CHARACTERS_OCD2,
            ],
            Config::S2TW => vec![
                &S2TW_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &ST_PHRASES_OCD2,
                &ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2,
                &ST_CHARACTERS_OCD2,
                &TW_VARIANTS_PHRASES_OCD2,
                &TW_VARIANTS_OCD2,
            ],
            Config::TW2S => vec![
                &TW2S_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TS_PHRASES_OCD2,
                &TW_VARIANTS_REV_PHRASES_OCD2,
                &TW_VARIANTS_REV_OCD2,
                &TS_PHRASES_OCD2,
                &TS_CHARACTERS_EXT_OCD2,
                &TS_CHARACTERS_OCD2,
            ],
            Config::S2HK => vec![
                &S2HK_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &ST_PHRASES_OCD2,
                &ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2,
                &ST_CHARACTERS_OCD2,
                &HK_VARIANTS_PHRASES_OCD2,
                &HK_VARIANTS_OCD2,
            ],
            Config::HK2S => vec![
                &HK2S_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TS_PHRASES_OCD2,
                &HK_VARIANTS_REV_PHRASES_OCD2,
                &HK_VARIANTS_REV_OCD2,
                &TS_PHRASES_OCD2,
                &TS_CHARACTERS_EXT_OCD2,
                &TS_CHARACTERS_OCD2,
            ],
            Config::S2TWP => vec![
                &S2TWP_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &ST_PHRASES_OCD2,
                &ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2,
                &ST_CHARACTERS_OCD2,
                &TW_PHRASES_OCD2,
                &TW_VARIANTS_PHRASES_OCD2,
                &TW_VARIANTS_OCD2,
            ],
            Config::TW2SP => vec![
                &TW2SP_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TS_PHRASES_OCD2,
                &TW_PHRASES_REV_OCD2,
                &TW_VARIANTS_REV_PHRASES_OCD2,
                &TW_VARIANTS_REV_OCD2,
                &TS_PHRASES_OCD2,
                &TS_CHARACTERS_EXT_OCD2,
                &TS_CHARACTERS_OCD2,
            ],
            Config::T2TW => vec![
                &T2TW_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TW_VARIANTS_PHRASES_OCD2,
                &TW_VARIANTS_OCD2,
            ],
            Config::TW2T => vec![
                &TW2T_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TW_VARIANTS_REV_PHRASES_OCD2,
                &TW_VARIANTS_REV_OCD2,
            ],
            Config::T2HK => vec![
                &T2HK_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &HK_VARIANTS_PHRASES_OCD2,
                &HK_VARIANTS_OCD2,
            ],
            Config::HK2T => vec![
                &HK2T_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &HK_VARIANTS_REV_PHRASES_OCD2,
                &HK_VARIANTS_REV_OCD2,
            ],
            Config::S2HKP => vec![
                &S2HKP_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &ST_PHRASES_OCD2,
                &ST_PHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD2,
                &ST_CHARACTERS_OCD2,
                &HK_PHRASES_OCD2,
                &HK_VARIANTS_PHRASES_OCD2,
                &HK_VARIANTS_OCD2,
            ],
            Config::HK2SP => vec![
                &HK2SP_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &TS_PHRASES_OCD2,
                &HK_PHRASES_REV_OCD2,
                &HK_VARIANTS_REV_PHRASES_OCD2,
                &HK_VARIANTS_REV_OCD2,
                &TS_PHRASES_OCD2,
                &TS_CHARACTERS_EXT_OCD2,
                &TS_CHARACTERS_OCD2,
            ],
            Config::T2JP => vec![
                &T2JP_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &JP_SHINJITAI_CHARACTERS_REV_OCD2,
            ],
            Config::JP2T => vec![
                &JP2T_JSON,
                &CJK_COMPATIBILITY_IDEOGRAPHS_OCD2,
                &JP_SHINJITAI_PHRASES_OCD2,
                &JP_SHINJITAI_CHARATERS_OCD2,
            ],
        }
    }
}

/// OpenCC error
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to create opencc instance
    #[error("Failed to create opencc instance")]
    Create,

    /// Failed to convert the string
    #[error("Failed to convert the string")]
    Convert,

    /// IO error
    #[error(transparent)]
    StdIO(#[from] io::Error),
}

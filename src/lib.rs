//! OpenCC bindings for Rust

use std::{
    ffi::{CStr, CString, c_void},
    fs, io,
};

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
    /// Traditional Chinese (Hong Kong Standard) to Simplified Chinese
    HK2S,
    /// Traditional Chinese (Hong Kong Standard) to Traditional Chinese
    HK2T,
    /// New Japanese Kanji (Shinjitai) to Traditional Chinese Characters (Kyūjitai)
    JP2T,
    /// Simplified Chinese to Traditional Chinese
    S2T,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    S2TW,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard) with Taiwanese idiom
    S2TWP,
    /// Traditional Chinese (OpenCC Standard) to Hong Kong Standard
    T2HK,
    /// Traditional Chinese Characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    T2JP,
    /// Traditional Chinese (OpenCC Standard) to Taiwan Standard
    T2TW,
    /// Traditional Chinese to Simplified Chinese
    T2S,
    /// Simplified Chinese to Traditional Chinese (Hong Kong Standard)
    S2HK,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese
    TW2S,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese with Mainland Chinese idiom
    TW2SP,
    /// Traditional Chinese (Taiwan Standard) to Traditional Chinese
    TW2T,
}

impl Config {
    fn get_data(&self) -> Vec<&opencc_sys::Data> {
        match self {
            Config::HK2S => vec![
                &opencc_sys::HK2S_JSON,
                &opencc_sys::TSPHRASES_OCD2,
                &opencc_sys::HKVARIANTS_REV_PHRASES_OCD2,
                &opencc_sys::HKVARIANTS_REV_OCD2,
                &opencc_sys::TSCHARACTERS_OCD2,
            ],
            Config::HK2T => vec![
                &opencc_sys::HK2T_JSON,
                &opencc_sys::HKVARIANTS_REV_PHRASES_OCD2,
                &opencc_sys::HKVARIANTS_REV_OCD2,
            ],
            Config::JP2T => vec![
                &opencc_sys::JP2T_JSON,
                &opencc_sys::JPSHINJITAI_PHRASES_OCD2,
                &opencc_sys::JPSHINJITAI_CHARATERS_OCD2,
                &opencc_sys::JPVARIANTS_REV_OCD2,
            ],
            Config::S2HK => vec![
                &opencc_sys::S2HK_JSON,
                &opencc_sys::STPHRASES_OCD2,
                &opencc_sys::STCHARACTERS_OCD2,
                &opencc_sys::HKVARIANTS_OCD2,
            ],
            Config::S2T => vec![
                &opencc_sys::S2T_JSON,
                &opencc_sys::STPHRASES_OCD2,
                &opencc_sys::STCHARACTERS_OCD2,
            ],
            Config::S2TW => vec![
                &opencc_sys::S2TW_JSON,
                &opencc_sys::STPHRASES_OCD2,
                &opencc_sys::STCHARACTERS_OCD2,
                &opencc_sys::TWVARIANTS_OCD2,
            ],
            Config::S2TWP => vec![
                &opencc_sys::S2TWP_JSON,
                &opencc_sys::STPHRASES_OCD2,
                &opencc_sys::STCHARACTERS_OCD2,
                &opencc_sys::TWPHRASES_OCD2,
                &opencc_sys::TWVARIANTS_OCD2,
            ],
            Config::T2HK => vec![&opencc_sys::T2HK_JSON, &opencc_sys::HKVARIANTS_OCD2],
            Config::T2JP => vec![&opencc_sys::T2JP_JSON, &opencc_sys::JPVARIANTS_OCD2],
            Config::T2S => vec![
                &opencc_sys::T2S_JSON,
                &opencc_sys::TSPHRASES_OCD2,
                &opencc_sys::TSCHARACTERS_OCD2,
            ],
            Config::T2TW => vec![&opencc_sys::T2TW_JSON, &opencc_sys::TWVARIANTS_OCD2],
            Config::TW2S => vec![
                &opencc_sys::TW2S_JSON,
                &opencc_sys::TSPHRASES_OCD2,
                &opencc_sys::TWVARIANTS_REV_PHRASES_OCD2,
                &opencc_sys::TWVARIANTS_REV_OCD2,
                &opencc_sys::TSCHARACTERS_OCD2,
            ],
            Config::TW2SP => vec![
                &opencc_sys::TW2SP_JSON,
                &opencc_sys::TSPHRASES_OCD2,
                &opencc_sys::TWPHRASES_REV_OCD2,
                &opencc_sys::TWVARIANTS_REV_PHRASES_OCD2,
                &opencc_sys::TWVARIANTS_REV_OCD2,
                &opencc_sys::TSCHARACTERS_OCD2,
            ],
            Config::TW2T => vec![
                &opencc_sys::TW2T_JSON,
                &opencc_sys::TWVARIANTS_REV_PHRASES_OCD2,
                &opencc_sys::TWVARIANTS_REV_OCD2,
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

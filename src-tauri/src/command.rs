use std::fs;
use tauri::{State, InvokeError};
use xuexi::definition::Definition;
use xuexi::export;
use crate::state::{Data, Language};
use crate::error::Error;

/// Set the language. This will be used by the app
/// to select which dictionnary to use to analyze the given text
/// 
/// # Arguments
/// 
/// * `&str` - lang
/// * `state` - State<Data>
#[tauri::command]
pub fn set_language(lang: &str, state: State<Data>) {
    // lock the mutex
    let mut lock_language = state.targeted_language
        .lock()
        .expect("Expect to acquired lock to change the language");

    match lang {
        "traditional_chinese" => *lock_language = Language::TraditionalChinese,
        "simplified_chinese" => *lock_language = Language::SimplifiedChinese,
        "laotian" => *lock_language = Language::Laotian,
        _ => *lock_language = Language::Laotian
    }

    // lock is drop at the end of the function
}

/// Generate the definition from a given string slice content
/// 
/// # Arguments
/// 
/// * `&str` - content
/// * `state` - State<Data>
#[tauri::command]
pub fn generate_definitions(content: &str, state: State<Data>) -> Result<Vec<Definition>, InvokeError> {
    state.get_detected_word_list(content)
        .map_err(InvokeError::from)
}

/// Export the definitions into a CSV
/// 
/// # Arguments
/// 
/// * `defs` - Vec<Definition>
/// * `path` - &str
#[tauri::command]
pub fn export_definition_to_csv(defs: Vec<Definition>, path: &str) -> Result<(), InvokeError> {
    let csv = export::export_to_csv(defs)
        .map_err(|err| InvokeError::from(Error::Export(err.to_string())))?;

    fs::write(path, csv)
        .map_err(|err| InvokeError::from(Error::IO(err.to_string())))?;

    Ok(())
}
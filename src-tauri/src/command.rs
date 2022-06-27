use tauri::State;
use crate::state::{Data, Language};

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
        "chinese" => *lock_language = Language::Chinese,
        "laotian" => *lock_language = Language::Laotian,
        _ => *lock_language = Language::Laotian
    }

    // lock is drop at the end of the function
}
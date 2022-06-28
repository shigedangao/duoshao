use std::sync::Mutex;
use xuexi::chinese::Dictionnary as CNDictionnary;
use xuexi::laotian::Dictionnary as LaoDictionnary;
use xuexi::common::{DetectWord, Ops};
use xuexi::definition::Definition;
use crate::error::Error;

pub enum Language {
    Chinese,
    Laotian
}

impl Default for Language {
    fn default() -> Self {
        Self::Chinese
    }
}

#[derive(Default)]
pub struct Data {
  pub chinese: CNDictionnary,
  pub laotian: LaoDictionnary,
  pub targeted_language: Mutex<Language>,
  pub text: Mutex<String>
}

impl Data {
    /// Initialize a new state. So far we're loading the two language at the same time
    /// in the memory. Maybe it could be better to initialize them later when the user select one
    /// of the language
    pub fn new() -> Self {
        let chinese = xuexi::load_chinese_dictionnary();
        let laotian = xuexi::load_laotian_dictionnary()
            .expect("Expect to load laotian dictionary");

        Data {
            chinese,
            laotian,
            targeted_language: Mutex::new(Language::default()),
            text: Mutex::new(String::new())
        }
    }

    /// Get a list of detected word based on the selected language / method
    /// 
    /// # Arguments
    /// 
    /// * `&str` - content 
    pub fn get_detected_word_list(&self, content: &str) -> Result<Vec<Definition>, Error> {
        let language_lock = self.targeted_language
            .lock()
            .map_err(|_| Error::Lock("language lock".to_string()))?;

        let res = match *language_lock {
            Language::Chinese => self.chinese.get_list_detected_words(content),
            Language::Laotian => self.laotian.get_list_detected_words(content)
        };

        if let Some(def) = res {
            let ordered_content: Vec<Definition> = def
                .get_ordered_characters()
                .into_iter()
                .map(|(_, def)| def)
                .collect();

            return Ok(ordered_content);
        }

        Err(Error::NoDefinitions)
    }
}
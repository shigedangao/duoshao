use std::sync::Mutex;
use xuexi::chinese::Dictionnary as CNDictionnary;
use xuexi::laotian::Dictionnary as LaoDictionnary;

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
}
use std::sync::Mutex;
use xuexi::chinese::Dictionary as CNDictionnary;
use xuexi::chinese::Version;
use xuexi::laotian::Dictionary as LaoDictionnary;
use xuexi::word::DetectWord;
use xuexi::ordering::Ops;
use xuexi::definition::Definition;
use crate::error::Error;
use crate::loader;
use crate::loader::load_other_dictionary;

pub enum Language {
    TraditionalChinese,
    SimplifiedChinese,
    Laotian
}

impl Default for Language {
    fn default() -> Self {
        Self::TraditionalChinese
    }
}

#[derive(Default)]
pub struct Data {
  pub traditional_chinese: CNDictionnary,
  pub simplified_chinese: CNDictionnary,
  pub laotian: LaoDictionnary,
  pub targeted_language: Mutex<Language>,
  pub text: Mutex<String>
}

impl Data {
    /// Initialize a new state. So far we're loading the two language at the same time
    /// in the memory. Maybe it could be better to initialize them later when the user select one
    /// of the language
    pub async fn new() -> Self {
        // Load the dictionary asynchronously
        let (tc, sc, lao) = tokio::try_join!(
            loader::load_cn_dictionary(xuexi::load_chinese_dictionary, Some(Version::Traditional)),
            loader::load_cn_dictionary(xuexi::load_chinese_dictionary, Some(Version::Simplified)),
            load_other_dictionary( xuexi::load_laotian_dictionary)
        ).expect("Expect to load dictionaries");

        Data {
            traditional_chinese: tc,
            simplified_chinese: sc,
            laotian: lao,
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
            Language::TraditionalChinese => self.traditional_chinese.get_list_detected_words(content),
            Language::SimplifiedChinese => self.simplified_chinese.get_list_detected_words(content),
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
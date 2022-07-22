use xuexi::error::LibError;
use xuexi::chinese::Version;
use xuexi::word::DetectWord;
use crate::error::Error;

/// Wrapper to load the chinese dictionary asynchronously
/// 
/// # Arguments
/// 
/// * `f` - F
/// * `version` - Option<Version>
pub async fn load_cn_dictionary<F, T>(f: F, version: Option<Version>) -> Result<T, Error>
    where
        F: Fn(Option<Version>) -> Result<T, LibError>,
        T: DetectWord
{
    let res = f(version)
        .map_err(|err| Error::Load(err.to_string()))?;
    
    Ok(res)
}

/// Wrapper to load other language dictionary asynchronously
/// 
/// # Arguments
/// 
/// * `f` - F
pub async fn load_other_dictionary<F, T>(f: F) -> Result<T, Error>
    where
        F: Fn() -> Result<T, LibError>,
        T: DetectWord
{
    let res = f()
        .map_err(|err| Error::Load(err.to_string()))?;

    Ok(res)
}
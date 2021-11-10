use crate::error::{self, UrlExtractError};

#[derive(Debug, PartialEq, Eq)]
pub struct Webtoon {
    pub title_no: usize,
    pub name: String,
    pub genre: String,
}

fn extract_url(url: &str) -> Result<Webtoon, error::UrlExtractError> {
    if !url.contains("list") {
        return Err(UrlExtractError::InvalidUrl);
    }

    let mut split: Vec<&str> = url.split('/').collect();
    split.reverse();

    let title_no: usize = split
        .get(0)
        .ok_or(UrlExtractError::InvalidUrl)?
        .strip_prefix("list?title_no=")
        .ok_or(UrlExtractError::InvalidUrl)?
        .parse()?;

    let name = split.get(1).ok_or(UrlExtractError::InvalidUrl)?.to_string();
    let genre = split.get(2).ok_or(UrlExtractError::InvalidUrl)?.to_string();

    Ok(Webtoon {
        title_no,
        name,
        genre,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let url = "https://www.webtoons.com/en/slice-of-life/boyfriends/list?title_no=2616";

        assert_eq!(
            Webtoon {
                title_no: 2616,
                name: "boyfriends".to_string(),
                genre: "slice-of-life".to_string(),
            },
            extract_url(url).unwrap()
        )
    }

    #[test]
    fn test_invalid() {
        let url = "https://some/invalid/url/of/sorts";

        assert!(extract_url(url).is_err())
    }

    #[test]
    fn test_episode() {
        let url = "https://www.webtoons.com/en/slice-of-life/boyfriends/episode-78/viewer?title_no=2616&episode_no=78";

        assert!(extract_url(url).is_err())
    }
}

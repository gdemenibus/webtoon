use crate::error::{self, UrlExtractError};

#[derive(Debug, PartialEq, Eq)]
pub struct Webtoon {
    pub title_no: usize,
    pub name: String,
    pub genre: String,
}

pub fn extract_url(url: &str) -> Result<Webtoon, error::UrlExtractError> {
    let mut split = url.split('/').rev();

    let title_no: usize = if url.contains("viewer") { // If Episode
        let title_no = split
            .next()
            .ok_or(UrlExtractError::InvalidUrl)?
            .split(&['?', '&'][..])
            .nth(1)
            .ok_or(UrlExtractError::InvalidUrl)?
            .strip_prefix("title_no=")
            .ok_or(UrlExtractError::InvalidUrl)?
            .parse()?;

        // Skip over `/episode-N/` part of the url
        split.next();

        title_no
    } else { // If List page
        split
            .next()
            .ok_or(UrlExtractError::InvalidUrl)?
            .strip_prefix("list?title_no=")
            .ok_or(UrlExtractError::InvalidUrl)?
            .parse()?
    };

    let name = split.next().ok_or(UrlExtractError::InvalidUrl)?.to_string();
    let genre = split.next().ok_or(UrlExtractError::InvalidUrl)?.to_string();

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

        assert_eq!(
            Webtoon {
                title_no: 2616,
                name: "boyfriends".to_string(),
                genre: "slice-of-life".to_string(),
            },
            extract_url(url).unwrap()
        );
    }
}

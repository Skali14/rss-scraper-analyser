use std::fmt::{Debug, Formatter, Pointer};
use regex::Regex;
use crate::translator;
use crate::translator::{FromLanguage, ToLanguage};

pub struct NewsArticle {
    pub(crate) headline: String,
    pub(crate) date: String,
    pub(crate) url: String,
    pub(crate) subject: String,
}

impl NewsArticle {
    pub fn extract_headlines(articles: Vec<NewsArticle>, translate: bool) -> Vec<String> {
        let re = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
        if translate {
            translator::translate(articles.into_iter()
                .map(|article| re.replace_all(&article.headline, "").to_string())
                .collect(), ToLanguage::ENUS)
        } else {
            articles.into_iter()
                .map(|article| re.replace_all(&article.headline, "").to_string())
                .collect()
        }
    }
}

impl Debug for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewsArticle")
            .field("headline", &self.headline)
            .field("date", &self.date)
            .field("url", &self.url)
            .field("subject", &self.subject)
            .finish()
    }
}

impl Default for NewsArticle {
    fn default() -> Self {
        NewsArticle {headline: String::from("Default"), date: String::from("Default"), url: String::from("Default"), subject: String::from("Default")}
    }
}

impl Clone for NewsArticle {
    fn clone(&self) -> Self {
        NewsArticle {
            headline: self.headline.clone(),
            date: self.date.clone(),
            url: self.url.clone(),
            subject: self.subject.clone(),
        }
    }
}
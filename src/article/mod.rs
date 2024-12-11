use std::fmt::{Debug, Formatter, Pointer};

pub struct NewsArticle {
    pub(crate) headline: String,
    pub(crate) date: String,
    pub(crate) url: String,
    pub(crate) subject: String
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
            subject: self.subject.clone()
        }
    }
}
use crate::translator;
use crate::article::NewsArticle;
use crate::translator::{FromLanguage, ToLanguage};
use crate::vader_sentiment::get_sentiment_value_single;

fn get_rss_feeds() -> String {
    let url = "https://ct24.ceskatelevize.cz/rss/tema/vyber-redakce-84313";
    reqwest::blocking::get(url).unwrap().text().unwrap()
}

pub fn get_articles() -> Vec<NewsArticle> {
    let feed = get_rss_feeds();
    let mut articles: Vec<NewsArticle> = Vec::default();

    let doc = roxmltree::Document::parse(feed.as_str()).unwrap();
    for item in doc.descendants().filter(|n| n.has_tag_name("channel")) {
        for item in item.descendants().filter(|n| n.has_tag_name("item")) {
            let headline = item.descendants().find(|n| n.has_tag_name("title")).unwrap().text().unwrap().to_owned();
            let url = item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap().to_owned();
            let date = item.descendants().find(|n| n.has_tag_name("pubDate")).unwrap().text().unwrap().to_owned();
            let subject = item.descendants().find(|n| n.has_tag_name("category")).unwrap().text().unwrap().to_owned();
            articles.push(NewsArticle {headline, date, url, subject})
        }
    }
    articles
}

/*pub fn get_headlines(translate: bool) -> Vec<String> {
    let mut headlines: Vec<String> = Vec::default();
    for article in get_articles() {
        headlines.push(article.clone().headline.replace("„", "").replace("“", "").to_string());
    }
    if translate {
        translator::translate(headlines, FromLanguage::CS, ToLanguage::ENUS)
    } else {
        headlines
    }
}*/
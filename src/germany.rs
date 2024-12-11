use regex::Regex;
use crate::translator;
use crate::article::NewsArticle;
use crate::translator::{FromLanguage, ToLanguage};

fn get_rss_feeds() -> Vec<String> {

    let urls = vec!["https://www.tagesschau.de/inland/index~rss2.xml", "https://www.tagesschau.de/ausland/index~rss2.xml", "https://www.tagesschau.de/wirtschaft/index~rss2.xml", "https://www.tagesschau.de/wissen/index~rss2.xml"];
    let mut results: Vec<String> = Vec::default();
    for url in urls {
        results.push(reqwest::blocking::get(url).unwrap().text().unwrap());
    }
    results
}

fn get_articles() -> Vec<NewsArticle> {
    let feeds = get_rss_feeds();
    let mut articles: Vec<NewsArticle> = Vec::default();
    let category_regex = Regex::new(r"https://www\.tagesschau\.de/([^/]+)/").unwrap();

    for feed in feeds {
        let doc = roxmltree::Document::parse(feed.as_str()).unwrap();
        for item in doc.descendants().filter(|n| n.has_tag_name("channel")) {
            for item in item.descendants().filter(|n| n.has_tag_name("item")) {
                let headline = item.descendants().find(|n| n.has_tag_name("title")).unwrap().text().unwrap().to_owned();
                let url = item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap().to_owned();
                let date = item.descendants().find(|n| n.has_tag_name("pubDate")).unwrap().text().unwrap().to_owned();
                let subject: String;
                if let Some(caps) = category_regex.captures(item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap()) {
                    subject = caps[1].to_owned();
                } else {
                    subject = String::from("n/a");
                }

                articles.push(NewsArticle {headline, date, url, subject})
            }
        }
    }
    articles
}

pub fn get_headlines(translate: bool) -> Vec<String> {
    let mut headlines: Vec<String> = Vec::default();
    for article in get_articles() {
        headlines.push(article.clone().headline.replace("\"", "").to_string());
    }
    if translate {
        translator::translate(headlines, FromLanguage::DE, ToLanguage::ENUS)
    } else {
        headlines
    }
}
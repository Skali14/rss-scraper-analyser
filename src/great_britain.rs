use regex::Regex;
use crate::article::NewsArticle;

fn get_rss_feeds() -> Vec<String> {

    let urls = vec!["https://feeds.bbci.co.uk/news/world/rss.xml", "https://feeds.bbci.co.uk/news/uk/rss.xml", "https://feeds.bbci.co.uk/news/business/rss.xml", "https://feeds.bbci.co.uk/news/politics/rss.xml", "https://feeds.bbci.co.uk/news/health/rss.xml", "https://feeds.bbci.co.uk/news/education/rss.xml", "https://feeds.bbci.co.uk/news/science_and_environment/rss.xml", "https://feeds.bbci.co.uk/news/technology/rss.xml", "https://feeds.bbci.co.uk/news/entertainment_and_arts/rss.xml"];
    let mut results: Vec<String> = Vec::default();
    for url in urls {
        results.push(reqwest::blocking::get(url).unwrap().text().unwrap());
    }
    results
}

pub fn get_articles() -> Vec<NewsArticle> {
    let feeds = get_rss_feeds();
    let mut articles: Vec<NewsArticle> = Vec::default();
    let category_regex = Regex::new(r"https://www\.bbc\.(com|co\.uk)/[^/]+/([^/]+)").unwrap();

    for feed in feeds {
        let doc = roxmltree::Document::parse(feed.as_str()).unwrap();
        for item in doc.descendants().filter(|n| n.has_tag_name("channel")) {

            let subject: String;
            if let Some(caps) = category_regex.captures(item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap()) {
                subject = caps[2].to_owned();
            } else {
                subject = String::from("n/a");
            }

            for item in item.descendants().filter(|n| n.has_tag_name("item")) {
                let headline = item.descendants().find(|n| n.has_tag_name("title")).unwrap().text().unwrap().to_owned();
                let url = item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap().to_owned();
                let date = item.descendants().find(|n| n.has_tag_name("pubDate")).unwrap().text().unwrap().to_owned();

                articles.push(NewsArticle {headline, date, url, subject: subject.clone()})
            }
        }
    }
    articles
}
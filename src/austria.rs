use crate::article::NewsArticle;

fn get_rss_feeds() -> (Vec<String>, Vec<String>) {
    let urls_normal = vec!["https://rss.orf.at/news.xml", "https://rss.orf.at/sport.xml", "https://rss.orf.at/oe3.xml", "https://rss.orf.at/fm4.xml"];
    let urls_abnormal = vec!["https://rss.orf.at/help.xml", "https://rss.orf.at/science.xml"];
    let mut results_normal: Vec<String> = Vec::default();
    let mut results_abnormal: Vec<String> = Vec::default();
    for url in urls_normal {
        results_normal.push(reqwest::blocking::get(url).unwrap().text().unwrap());
    }
    for url in urls_abnormal {
        results_abnormal.push(reqwest::blocking::get(url).unwrap().text().unwrap());
    }
    (results_normal, results_abnormal)
}

pub fn get_articles() -> Vec<NewsArticle> {
    let (feeds_normal, feeds_abnormal) = get_rss_feeds();
    let mut articles: Vec<NewsArticle> = Vec::default();
    for feed in feeds_normal {
        let doc = roxmltree::Document::parse(feed.as_str()).unwrap();

        for item in doc.descendants().filter(|n| n.has_tag_name("item")) {
            let headline = item.descendants().find(|n| n.has_tag_name("title")).unwrap().text().unwrap().to_owned();
            let url = item.descendants().find(|n| n.has_tag_name("link")).unwrap().text().unwrap().to_owned();
            let date = item.descendants().find(|n| n.has_tag_name("date")).unwrap().text().unwrap().to_owned();
            let subject = item.descendants().find(|n| n.has_tag_name("subject")).unwrap().text().unwrap().to_owned();

            articles.push(NewsArticle {headline, date, url, subject})
        }
    }

    for feed in feeds_abnormal {
        let doc = roxmltree::Document::parse(feed.as_str()).unwrap();
        for item in doc.descendants().filter(|n| n.has_tag_name("channel")) {
            for item in item.descendants().filter(|n| n.has_tag_name("item")) {
                let headline = item.descendants().find(|n| n.has_tag_name("title")).unwrap().text().unwrap().to_owned();
                let url = item.descendants().find(|n| n.has_tag_name("guid")).unwrap().text().unwrap().to_owned();
                let date = item.descendants().find(|n| n.has_tag_name("pubDate")).unwrap().text().unwrap().to_owned();
                let subject = item.descendants().find(|n| n.has_tag_name("category")).unwrap().text().unwrap().to_owned();
                articles.push(NewsArticle {headline, date, url, subject})
            }
        }
    }
    articles.clone().to_vec()
}
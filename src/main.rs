mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod openai;
mod article;
mod wordcloud;
mod azure_sentiment;
mod vader_sentiment;

use text_io::read;
use crate::article::NewsArticle;
use crate::azure_sentiment::get_sentiment_scores;
use crate::wordcloud::generate_wordcloud;

fn main() {
    /*loop {
        println!("Choose from these options:");
        println!("-------------------------\n");
        println!("1 - Start RSS Feed Analyzer");
        println!("2 - Start RSS Feed Analyzer (disabled translation)");
        println!("3 - Quit");

        let x: u32 = read!();

        match x {
            1 => {todo!(); break; }
            2 => {todo!(); break; }
            3 => return,
            _ => println!("Please try again!")
        }
    }*/
    //dbg!(austria::get_headlines(false));
    dbg!(vader_sentiment::get_sentiment_value_multiple(NewsArticle::extract_headlines(austria::get_articles(), true)));
    //dbg!(vader_sentiment::get_sentiment_value_single(&String::from("The weather is really bad today")));
    //dbg!(get_sentiment_scores(austria::get_headlines(false)));
    /*generate_wordcloud(austria::get_headlines(false), "austria_wordcloud.png");
    generate_wordcloud(germany::get_headlines(false), "germany_wordcloud.png");
    generate_wordcloud(czechia::get_headlines(false), "czechia_wordcloud.png");
    generate_wordcloud(france::get_headlines(false), "france_wordcloud.png");*/
}

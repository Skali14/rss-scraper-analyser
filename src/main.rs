mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod article;
mod wordcloud;
mod vader_sentiment;
mod charts;

use rand::Rng;
use text_io::read;
use crate::article::NewsArticle;
use crate::wordcloud::generate_wordcloud;
use crate::charts::{get_chart_multiple_dataset_4, get_chart_single_dataset};

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

    /*let data = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    if let Err(e) = generate_boxplot(data, "boxplot.png") {
        eprintln!("Error generating boxplot: {}", e);
    }*/

    //chart(vec![vader_sentiment::get_sentiment_value_multiple(NewsArticle::extract_headlines(austria::get_articles(), true)), vader_sentiment::get_sentiment_value_multiple(NewsArticle::extract_headlines(germany::get_articles(), true))], vec!["Austria", "Germany"]);
    let mut rng = rand::thread_rng();
    let data_multiple: Vec<Vec<f64>> = (0..4)
        .map(|_| (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect())
        .collect();

    let data_single: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();

    get_chart_single_dataset(data_single, "Austria");
    get_chart_multiple_dataset_4(data_multiple, vec!["Austria", "Germany", "France", "Czechia"]);


    //dbg!(austria::get_headlines(false));
    //dbg!(vader_sentiment::get_sentiment_value_multiple(NewsArticle::extract_headlines(austria::get_articles(), true)));
    //dbg!(vader_sentiment::get_sentiment_value_single(&String::from("The weather is really bad today")));
    //dbg!(get_sentiment_scores(austria::get_headlines(false)));
    /*generate_wordcloud(austria::get_headlines(false), "austria_wordcloud.png");
    generate_wordcloud(germany::get_headlines(false), "germany_wordcloud.png");
    generate_wordcloud(czechia::get_headlines(false), "czechia_wordcloud.png");
    generate_wordcloud(france::get_headlines(false), "france_wordcloud.png");*/
}

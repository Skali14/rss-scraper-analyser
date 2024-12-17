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
use std::fs;
use text_io::read;
use crate::article::NewsArticle;
use crate::wordcloud::generate_wordcloud;
use crate::charts::{get_chart_multiple_dataset_4, get_chart_single_dataset};
use crate::vader_sentiment::get_sentiment_value_multiple;

fn main() {
    loop {
        println!("Choose from these options:");
        println!("-------------------------\n");
        println!("1 - Run RSS Feed Analyzer (complete with translation)");
        println!("2 - Run RSS Feed Analyzer (disabled translation)");
        println!("3 - Run RSS Feed Analyzer (only english news sites / disabled translation)");
        println!("4 - Demo (wordcloud without translation / sentiment analysis with random values)");
        println!("5 - Quit");

        let x: u32 = read!();

        delete_previous_files();

        match x {
            1 => {
                run_complete(true);
                break;
            }
            2 => {
                run_complete(false);
                break;
            }
            3 => {
                run_only_en();
                break;
            }
            4 => {
                run_demo();
                break;
            }
            5 => return,
            _ => println!("Please try again!")
        }
    }
}

fn delete_previous_files() {
    let files_to_delete = vec![
        "boxplot_multiple.html",
        "boxplot_austria.html",
        "boxplot_germany.html",
        "boxplot_france.html",
        "boxplot_czechia.html",
        "all_wordcloud.png",
        "austria_wordcloud.png",
        "germany_wordcloud.png",
        "france_wordcloud.png",
        "czechia_wordcloud.png",
    ];

    for file in files_to_delete {
        fs::remove_file(file).unwrap_or_else(|_| ());
    }
}

fn run_complete(translate: bool) {
    let austria_articles = austria::get_articles();
    let germany_articles = germany::get_articles();
    let france_articles = france::get_articles();
    let czechia_articles = czechia::get_articles();

    let austria_hl = NewsArticle::extract_headlines(austria_articles, translate);
    let germany_hl = NewsArticle::extract_headlines(germany_articles, translate);
    let france_hl = NewsArticle::extract_headlines(france_articles, translate);
    let czechia_hl = NewsArticle::extract_headlines(czechia_articles, translate);

    let vader_austria = get_sentiment_value_multiple(austria_hl.clone());
    let vader_germany = get_sentiment_value_multiple(germany_hl.clone());
    let vader_france = get_sentiment_value_multiple(france_hl.clone());
    let vader_czechia = get_sentiment_value_multiple(czechia_hl.clone());

    generate_wordcloud(vec![austria_hl.clone(), germany_hl.clone(), france_hl.clone(), czechia_hl.clone()].into_iter().flat_map(|v| v).collect(), "all_wordcloud.png");

    generate_wordcloud(austria_hl, "austria_wordcloud.png");
    generate_wordcloud(germany_hl, "germany_wordcloud.png");
    generate_wordcloud(france_hl, "france_wordcloud.png");
    generate_wordcloud(czechia_hl, "czechia_wordcloud.png");

    get_chart_multiple_dataset_4(vec![vader_austria.clone(), vader_germany.clone(), vader_france.clone(), vader_czechia.clone()], vec!["Austria", "Germany", "France", "Czechia"], "boxplot_multiple.html");

    get_chart_single_dataset(vader_austria, "Austria", "boxplot_austria.html");
    get_chart_single_dataset(vader_germany, "Germany", "boxplot_germany.html");
    get_chart_single_dataset(vader_france, "France", "boxplot_france.html");
    get_chart_single_dataset(vader_czechia, "Czechia", "boxplot_czechia.html");
}


fn run_only_en() {
    let austria_articles = austria::get_articles();

    let austria_hl = NewsArticle::extract_headlines(austria_articles, false);

    let vader_austria = get_sentiment_value_multiple(austria_hl.clone());

    generate_wordcloud(austria_hl, "austria_wordcloud.png");

    get_chart_single_dataset(vader_austria, "Austria", "boxplot_austria.html");
}


fn run_demo() {
    let austria_articles = austria::get_articles();
    let germany_articles = germany::get_articles();
    let france_articles = france::get_articles();
    let czechia_articles = czechia::get_articles();

    let austria_hl = NewsArticle::extract_headlines(austria_articles, false);
    let germany_hl = NewsArticle::extract_headlines(germany_articles, false);
    let france_hl = NewsArticle::extract_headlines(france_articles, false);
    let czechia_hl = NewsArticle::extract_headlines(czechia_articles, false);

    generate_wordcloud(vec![austria_hl.clone(), germany_hl.clone(), france_hl.clone(), czechia_hl.clone()].into_iter().flat_map(|v| v).collect(), "all_wordcloud.png");

    generate_wordcloud(austria_hl, "austria_wordcloud.png");
    generate_wordcloud(germany_hl, "germany_wordcloud.png");
    generate_wordcloud(france_hl, "france_wordcloud.png");
    generate_wordcloud(czechia_hl, "czechia_wordcloud.png");

    let mut rng = rand::thread_rng();
    let data_multiple: Vec<Vec<f64>> = (0..4)
        .map(|_| (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect())
        .collect();

    let data_single_1: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_2: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_3: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_4: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();

    get_chart_multiple_dataset_4(data_multiple, vec!["Austria", "Germany", "France", "Czechia"], "boxplot_multiple.html");

    get_chart_single_dataset(data_single_1, "Austria", "boxplot_austria.html");
    get_chart_single_dataset(data_single_2, "Germany", "boxplot_germany.html");
    get_chart_single_dataset(data_single_3, "France", "boxplot_france.html");
    get_chart_single_dataset(data_single_4, "Czechia", "boxplot_czechia.html");
}

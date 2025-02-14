mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod article;
mod wordcloud;
mod vader_sentiment;
mod charts;
mod great_britain;

use rand::Rng;
use std::fs;
use config_reader::read_value;
use text_io::read;
use crate::article::NewsArticle;
use crate::wordcloud::generate_wordcloud;
use crate::charts::{get_chart_multiple_dataset_5, get_chart_single_dataset};
use crate::vader_sentiment::get_sentiment_value_multiple;

fn main() {
    loop {
        println!("Choose from these options:");
        println!("-------------------------\n");
        println!("1 - Run RSS Feed Analyzer (complete with translation) [DEEPL API KEY REQUIRED]");
        println!("2 - Run RSS Feed Analyzer (complete without translation)");
        println!("3 - Run RSS Feed Analyzer (only english news sites / disabled translation)");
        println!("4 - Run Demo (wordcloud without translation / sentiment analysis with random values)");
        println!("5 - Quit");

        let x: u32 = read!();

        match x {
            1 => {
                if read_value("DEEPLAPIKEY").eq("<INSERT API KEY HERE>") {
                    eprintln!("DeepL Api Key is not set, aborting!");
                    return;
                }
                delete_previous_files();
                create_output_folder();
                run_complete(true);
                break;
            }
            2 => {
                delete_previous_files();
                create_output_folder();
                run_complete(false);
                break;
            }
            3 => {
                delete_previous_files();
                create_output_folder();
                run_only_en();
                break;
            }
            4 => {
                delete_previous_files();
                create_output_folder();
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
        "output/boxplot_multiple.html",
        "output/boxplot_austria.html",
        "output/boxplot_germany.html",
        "output/boxplot_france.html",
        "output/boxplot_czechia.html",
        "output/boxplot_gb.html",
        "output/all_wordcloud.png",
        "output/austria_wordcloud.png",
        "output/germany_wordcloud.png",
        "output/france_wordcloud.png",
        "output/czechia_wordcloud.png",
        "output/gb_wordcloud.png",
    ];

    for file in files_to_delete {
        fs::remove_file(file).unwrap_or_else(|_| ());
    }
}

fn create_output_folder() {
    fs::create_dir("output").unwrap_or_else(|_| ());
}

fn run_complete(translate: bool) {
    let austria_articles = austria::get_articles();
    let germany_articles = germany::get_articles();
    let france_articles = france::get_articles();
    let czechia_articles = czechia::get_articles();
    let gb_articles = great_britain::get_articles();

    let austria_hl = NewsArticle::extract_headlines(austria_articles, translate);
    let germany_hl = NewsArticle::extract_headlines(germany_articles, translate);
    let france_hl = NewsArticle::extract_headlines(france_articles, translate);
    let czechia_hl = NewsArticle::extract_headlines(czechia_articles, translate);
    let gb_hl = NewsArticle::extract_headlines(gb_articles, false);

    let vader_austria = get_sentiment_value_multiple(austria_hl.clone());
    let vader_germany = get_sentiment_value_multiple(germany_hl.clone());
    let vader_france = get_sentiment_value_multiple(france_hl.clone());
    let vader_czechia = get_sentiment_value_multiple(czechia_hl.clone());
    let vader_gb = get_sentiment_value_multiple(gb_hl.clone());

    generate_wordcloud(vec![austria_hl.clone(), germany_hl.clone(), france_hl.clone(), czechia_hl.clone(), gb_hl.clone()].into_iter().flat_map(|v| v).collect(), "output/all_wordcloud.png");

    generate_wordcloud(austria_hl, "output/austria_wordcloud.png");
    generate_wordcloud(germany_hl, "output/germany_wordcloud.png");
    generate_wordcloud(france_hl, "output/france_wordcloud.png");
    generate_wordcloud(czechia_hl, "output/czechia_wordcloud.png");
    generate_wordcloud(gb_hl, "output/gb_wordcloud.png");

    get_chart_multiple_dataset_5(vec![vader_austria.clone(), vader_germany.clone(), vader_france.clone(), vader_czechia.clone(), vader_gb.clone()], vec!["Austria", "Germany", "France", "Czechia", "Great Britain"], "output/boxplot_multiple.html");

    get_chart_single_dataset(vader_austria, "Austria", "output/boxplot_austria.html");
    get_chart_single_dataset(vader_germany, "Germany", "output/boxplot_germany.html");
    get_chart_single_dataset(vader_france, "France", "output/boxplot_france.html");
    get_chart_single_dataset(vader_czechia, "Czechia", "output/boxplot_czechia.html");
    get_chart_single_dataset(vader_gb, "Great Britain", "output/boxplot_gb.html")
}


fn run_only_en() {
    let gb_articles = great_britain::get_articles();

    let gb_hl = NewsArticle::extract_headlines(gb_articles, false);

    let vader_gb = get_sentiment_value_multiple(gb_hl.clone());

    generate_wordcloud(gb_hl.clone(), "output/gb_wordcloud.png");
    fs::copy("output/gb_wordcloud.png", "output/all_wordcloud.png").expect("copy failed");

    get_chart_single_dataset(vader_gb.clone(), "Great Britain", "output/boxplot_gb.html");
    fs::copy("output/boxplot_gb.html", "output/boxplot_multiple.html").expect("copy failed");
}


fn run_demo() {
    let austria_articles = austria::get_articles();
    let germany_articles = germany::get_articles();
    let france_articles = france::get_articles();
    let czechia_articles = czechia::get_articles();
    let gb_articles = great_britain::get_articles();

    let austria_hl = NewsArticle::extract_headlines(austria_articles, false);
    let germany_hl = NewsArticle::extract_headlines(germany_articles, false);
    let france_hl = NewsArticle::extract_headlines(france_articles, false);
    let czechia_hl = NewsArticle::extract_headlines(czechia_articles, false);
    let gb_hl = NewsArticle::extract_headlines(gb_articles, false);

    generate_wordcloud(vec![austria_hl.clone(), germany_hl.clone(), france_hl.clone(), czechia_hl.clone(), gb_hl.clone()].into_iter().flat_map(|v| v).collect(), "output/all_wordcloud.png");

    generate_wordcloud(austria_hl, "output/austria_wordcloud.png");
    generate_wordcloud(germany_hl, "output/germany_wordcloud.png");
    generate_wordcloud(france_hl, "output/france_wordcloud.png");
    generate_wordcloud(czechia_hl, "output/czechia_wordcloud.png");
    generate_wordcloud(gb_hl, "output/gb_wordcloud.png");

    let mut rng = rand::thread_rng();
    let data_multiple: Vec<Vec<f64>> = (0..5)
        .map(|_| (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect())
        .collect();

    let data_single_1: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_2: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_3: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_4: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let data_single_5: Vec<f64> = (0..10).map(|_| rng.gen_range(-1.0..1.0)).collect();

    get_chart_multiple_dataset_5(data_multiple, vec!["Austria", "Germany", "France", "Czechia", "Great Britain"], "output/boxplot_multiple.html");

    get_chart_single_dataset(data_single_1, "Austria", "output/boxplot_austria.html");
    get_chart_single_dataset(data_single_2, "Germany", "output/boxplot_germany.html");
    get_chart_single_dataset(data_single_3, "France", "output/boxplot_france.html");
    get_chart_single_dataset(data_single_4, "Czechia", "output/boxplot_czechia.html");
    get_chart_single_dataset(data_single_5, "Great Britain", "output/boxplot_gb.html");
}

mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod openai;
mod article;
mod wordcloud;

use text_io::read;
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
    generate_wordcloud(austria::get_headlines(true), "austria_wordcloud.png");
    generate_wordcloud(germany::get_headlines(true), "germany_wordcloud.png");
    generate_wordcloud(czechia::get_headlines(true), "czechia_wordcloud.png");
    generate_wordcloud(france::get_headlines(true), "france_wordcloud.png");
}

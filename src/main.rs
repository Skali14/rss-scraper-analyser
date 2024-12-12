mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod sentiment_analysis;
mod article;

use text_io::read;

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
    let test = france::get_headlines(false);
    dbg!(test);
}

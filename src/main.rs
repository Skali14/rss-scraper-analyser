mod czechia;
mod austria;
mod germany;
mod france;
mod translator;
mod sentiment_analysis;
mod article;

fn main() {
    let test = austria::get_articles();
    println!("{}", test.len());
    //dbg!(test);
}

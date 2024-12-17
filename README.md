# News Site Analyzer
This project will take several news sites from around europe, translating them to english if necessary 
using the deepL API (toggleable), and then analyze the news in different ways:
- A [wordcloud](https://crates.io/crates/wordcloud-rs) to show often used words excluding words without meaning
- Using [VADER](https://github.com/cjhutto/vaderSentiment) ([VADER: Rust Version](https://github.com/ckw017/vader-sentiment-rust)) to estimate the sentiment of the news headlines
- Generate a boxplot to visualize the sentiment data using [charming](https://crates.io/crates/charming)
- create a HTML site using CSS and JS to provide easy access to the visualizations

### Currently implemented RSS Feeds

- https://rss.orf.at/
- https://www.tagesschau.de/infoservices/rssfeeds
- https://www.francetvinfo.fr/rss/
- https://ct24.ceskatelevize.cz/rss/tema/vyber-redakce-84313
- https://www.bbc.co.uk/news/10628494

### Planned RSS Feeds
- TBD

## Installation
1. Install [rustup](https://rustup.rs/)


2. Clone the repository:
```bash
  git clone https://sourcery.im.jku.at/missing-semester-2024/ex1-k12222898.git
```


3. Copy [config_template.toml](config_template.toml) and rename it to ``config.toml``


4. (Optional) Register for a free API-Key on [DeepL](https://www.deepl.com/de/pro-api) and enter it in ``config.toml``


5. Use cargo to run the project
```bash
  cargo run --release
```

## Usage
1. After running the project, select what you want to run


2. After the program finished executing and generating the needed files, open submission.html in a browser and view the visualization.


## Important
When running the project without using the translation functionality, the VADER Sentiment Analysis and therefore the Boxplot visualization can and will be inaccurate, since it depends on english news headlines.

## Credits
Hutto, C.J. & Gilbert, E.E. (2014). VADER: A Parsimonious Rule-based Model for Sentiment Analysis of Social Media Text. Eighth International Conference on Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.
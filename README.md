# News Site Analyzer
This project will take several news sites from around europe, translating them to english if necessary 
using the deepL API, and then analyze the news in different ways, including, but not limited to:
- A wordcloud to show often used words
- A sentiment analysis using [Microsoft Azure Sentiment API](https://learn.microsoft.com/en-us/azure/ai-services/language-service/sentiment-opinion-mining/quickstart?tabs=windows&pivots=rest-api)
- more will follow as the project progresses...

### Currently implemented RSS Feeds

- TODO

### Planned RSS Feeds

- https://rss.orf.at/news.xml
- https://www.tagesschau.de/infoservices/alle-meldungen-100~rss2.xml
- https://www.lemonde.fr/en/rss/une.xml
- https://www.aktualne.cz/rss/

## Installation
1. Install [rustup](https://rustup.rs/)


2. Clone the repository:
```bash
  git clone https://sourcery.im.jku.at/missing-semester-2024/ex1-k12222898.git
```

## Usage
1. Use cargo to run the project
```bash
  cargo run --release
```
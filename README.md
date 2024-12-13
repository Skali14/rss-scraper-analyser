# News Site Analyzer
This project will take several news sites from around europe, translating them to english if necessary 
using the deepL API (toggleable), and then analyze the news in different ways:
- A wordcloud to show often used words
- Using the OpenAI API to guess the severity of the news headlines (toggleable)
- more will follow as the project progresses...

### Currently implemented RSS Feeds

- https://rss.orf.at/
- https://www.tagesschau.de/infoservices/rssfeeds
- https://www.francetvinfo.fr/rss/
- https://ct24.ceskatelevize.cz/rss/tema/vyber-redakce-84313

### Planned RSS Feeds



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
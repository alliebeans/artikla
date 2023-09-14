# WIP

### Artikla.
For personal use and educational purposes only.
---

Web app for personal use built with [rocket.rs](https://rocket.rs/) web framework, [tera](https://keats.github.io/tera/docs/) templating and [htmx](https://htmx.org/).

Articles are generated by taking data from a RSS feed and send it to a postgres DB using [sqlx](https://docs.rs/sqlx/latest/sqlx/).

Utilizes [@moz/readability](https://github.com/mozilla/readability) to fetch relevant text from article html docs.

TODO:
- Implement RSS channels to my liking.
- When creating Article, categorize into Topics.

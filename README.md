# WIP

### Artikla.
---
*Beginner project for educational purposes*

Fetches relevant text from articles with [@moz/readability](https://github.com/mozilla/readability).

Articles are generated by taking data from RSS feeds and populating database.

Built with [rocket.rs](https://rocket.rs/) web framework, [htmx](https://htmx.org/), [tera](https://keats.github.io/tera/docs/) templating and postgres DB using [sqlx](https://docs.rs/sqlx/latest/sqlx/).

TODO:
- Implement RSS channels to my liking.
- When creating Article, categorize into Topics.
- Cutomize readability fetch to all implemented article sources.
- Weather data from SMHI
- Account functionality: login authorization etc
- Settings functionality

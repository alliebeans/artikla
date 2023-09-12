/* import { Readability } from '/public/js/readability/Readability.js'; */
const urlItem = `articleUrl`;

function init() {
    loadFetcher();
}

function loadFetcher() {
    let uuid = document.getElementById(`full-article`).getAttribute(`data-uuid`);
    let body = `/articles/${uuid}/content/1`;
    fetchArticle(body);
}

function fetchArticle(body) {
    let node = document.getElementById(`full-article`);

    fetch(body)
    .then((response) => response.text())
    .then((text) => new DOMParser().parseFromString(text, "text/html"))
    .then((dom) => {
        let parsedArticle = new Readability(dom).parse();
        let articleContainer = document.createElement(`article`);
        articleContainer.setAttribute(`id`, `full-article-content`);
        let title = document.createElement(`h1`);
        title.innerHTML = parsedArticle.title;
        node.appendChild(articleContainer);
        let articleContent = parsedArticle.content;
        let articleTag = articleContent.split(`</article>`)[0];
        const articleParagraphs = [...articleTag.matchAll(`<\s*p[^>]*>([^<]*)<\s*\/\s*p\s*>`)];
        articleContainer.appendChild(title);
        let article = document.createElement(`article`);
        for(let i = 0; i < articleParagraphs.length; i++) {
            article.innerHTML += articleParagraphs[i][0];
        }
        
        articleContainer.appendChild(article);

    })
    .catch(`Could not load document.`);
}

document.body.addEventListener(`htmx:afterSwap`, ()=> {
    if (document.getElementById(`full-article`) !== null) {
        init();
    }
});
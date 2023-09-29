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

async function fetchArticle(body) {
    let node = document.getElementById(`full-article`);

    

    fetch(body)
    .then((response) => response.text())
    .then((text) => new DOMParser().parseFromString(text, "text/html"))
    .then((dom) => {
        // create HTMLElements
        let articleContainer = document.createElement(`article`);
        let title = document.createElement(`h1`);
        let article = document.createElement(`article`);

        // get all article content
        let parsedArticle = new Readability(dom).parse();
        let articleContent = parsedArticle.content;
        // filter contents
        let articleTag = articleContent.split(`</article>`)[0];
        const articleParagraphs = [...articleTag.matchAll(`<\s*p[^>]*>([^<]*)<\s*\/\s*p\s*>`)];
        // set final contents
        title.innerHTML = parsedArticle.title;
        for(let i = 0; i < articleParagraphs.length; i++) {
            article.innerHTML += articleParagraphs[i][0];
        }

        // append elements to DOM
        articleContainer.setAttribute(`id`, `full-article-content`);
        node.appendChild(articleContainer);
        articleContainer.appendChild(title);
        articleContainer.appendChild(article);
    })
    .catch(`Could not load document.`);
}

document.body.addEventListener(`htmx:afterSwap`, ()=> {
    if (document.getElementById(`full-article`) !== null) {
        init();
    }
});
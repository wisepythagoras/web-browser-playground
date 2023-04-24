use scraper::{Html, Selector};

use crate::html::script::Script;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Document<'a> {
    contents: &'a str,
    document: Html,
    pub scripts: Vec<Script>,
}

impl<'a> Document<'a> {
    pub(crate) fn init(html: &'a str) -> Document<'a> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("script").unwrap();
        let mut scripts: Vec<Script> = vec![];

        for el in document.select(&selector) {
            let attr = el.value().attr("src");

            if attr.is_none() {
                let inner = el.inner_html();
                scripts.push(Script {
                    source: inner,
                    src: String::new(),
                });
                continue;
            }

            scripts.push(Script {
                source: String::new(),
                src: String::from(attr.unwrap()),
            });
        }

        return Document {
            contents: html,
            document,
            scripts,
        };
    }
}

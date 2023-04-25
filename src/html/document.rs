use scraper::{ElementRef, Html, Selector};

use crate::html::script::Script;
use std::fs;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Document {
    contents: String,
    document: Html,
    pub scripts: Vec<Script>,
}

impl Document {
    /**
     * Creates a new HTML document and separates the JavaScript sources.
     */
    pub(crate) fn new(html: String) -> Document {
        let document = Html::parse_document(html.as_str());
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

    /**
     * This function returns a string that contains all of the JavaScript code.
     */
    pub(crate) fn get_js_source(&mut self) -> String {
        let mut js_data = String::new();

        for scr in self.scripts.clone() {
            if scr.src != "" {
                js_data = match fs::read_to_string(scr.src) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Error {}", err.kind().to_string());
                        String::new()
                    }
                };
            } else {
                js_data += scr.source.as_str()
            }
        }

        return js_data;
    }

    pub(crate) fn traverse(&self) {
        let root = self.document.root_element();
        self.traverse_node(root);
    }

    fn traverse_node(&self, el: ElementRef) {
        if el.value().name() == "script" {
            return;
        }

        for child in el.children() {
            if child.value().is_comment() || child.value().is_doctype() {
                continue;
            }

            match child.value().as_element() {
                Some(child_el) => {
                    println!("{} - {}", child_el.name(), child.has_children());

                    match ElementRef::wrap(child) {
                        Some(el_ref) => self.traverse_node(el_ref),
                        None => continue,
                    }
                }
                None => continue,
            };
        }
    }
}

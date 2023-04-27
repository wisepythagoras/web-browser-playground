mod js {
    pub mod clipboard;
    pub mod console;
    pub mod navigator;
    pub mod node;
}
mod html {
    pub mod document;
    pub mod script;
}

use boa_engine::{property::Attribute, Context, JsResult, JsValue, NativeFunction, Source};
use html::document;
use js::{clipboard::Clipboard, console::Console, navigator::Navigator, node::Node};
use std::{env, fs, future::Future, process};

fn test(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
    args.get(0)
        .cloned()
        .unwrap_or_default()
        .pow(&JsValue::new(2), context)
}

fn test2(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context<'_>,
) -> impl Future<Output = JsResult<JsValue>> {
    let arg = args.get(0).cloned();
    let res = arg.unwrap_or_default().pow(&JsValue::new(2), context);

    async move {
        std::future::ready(()).await;

        match res {
            Ok(v) => {
                println!("Res: {:?}", v);
                return Ok(v);
            }
            Err(_) => Ok(JsValue::undefined()),
        }
    }
}

fn init_browser(context: &mut Context, doc: Option<document::Document>) {
    context
        .register_global_builtin_callable("myfn", 0, NativeFunction::from_fn_ptr(test))
        .expect("Registers");
    context
        .register_global_builtin_callable("myfn2", 1, NativeFunction::from_async_fn(test2))
        .expect("Registers");

    let navigator = Navigator::init(context);

    match navigator {
        Some(val) => {
            context
                .register_global_property("navigator", val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning navigator"),
    };

    let console = Console::init(context);

    match console {
        Some(val) => {
            context
                .register_global_property("console", val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning console"),
    };

    let clipboard = Clipboard::init(context);

    match clipboard {
        Some(val) => {
            context
                .register_global_property("clipboard", val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning clipboard"),
    };

    // match doc {
    //     Some(document) => {
    //         let document_node = Node::init(context, &mut document.clone());
    //     }
    //     None => {}
    // };
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Usage:\n./web-browser --html file.html");
        println!("./web-browser --js file.js");
        process::exit(1);
    }

    let js_data;
    let filepath = args[1].clone();
    let mut html_doc: Option<document::Document> = None;

    if args[0] == "--html" {
        let mut doc = parse_html(filepath.as_str());
        html_doc = Some(doc.clone());
        js_data = doc.get_js_source();
        doc.traverse();
    } else {
        js_data = match fs::read_to_string(filepath) {
            Ok(data) => data,
            Err(err) => {
                println!("Error {}", err.kind().to_string());
                process::exit(1);
            }
        };
    }

    let mut context = Context::default();
    init_browser(&mut context, html_doc);

    let res = context.eval_script(Source::from_bytes(js_data.as_str()));
    context.run_jobs();

    match res {
        Ok(_) => println!("Script was run"),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn parse_html(filepath: &str) -> document::Document {
    let html = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(err) => {
            println!("Error {}", err.kind().to_string());
            process::exit(1);
        }
    };

    return document::Document::new(html);
}

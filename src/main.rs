mod js {
    pub mod clipboard;
    pub mod console;
    pub mod navigator;
    pub mod node;
    pub mod person;
    pub mod fetch;
    pub mod json;
}
mod html {
    pub mod document;
    pub mod script;
}

use boa_engine::{
    js_string,
    property::Attribute,
    Context,
    JsError,
    JsResult,
    JsString,
    JsValue,
    NativeFunction,
    Source
};
use boa_runtime::Console;
use html::document;
use js::{
    clipboard::Clipboard,
    navigator::Navigator,
    node::Node,
    person::Person,
    fetch::fetch_fn,
    json::JSON,
};
use std::{env, fs, future::Future, process};

fn test(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    args.get(0)
        .cloned()
        .unwrap_or_default()
        .pow(&JsValue::new(2), context)
}

fn test2(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
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
        .register_global_builtin_callable(JsString::from("myfn"), 0, NativeFunction::from_fn_ptr(test))
        .expect("Registers");
    context
        .register_global_builtin_callable(JsString::from("myfn2"), 1, NativeFunction::from_async_fn(test2))
        .expect("Registers");
    context
        .register_global_builtin_callable(JsString::from("fetch"), 1, NativeFunction::from_async_fn(fetch_fn))
        .expect("Registers");

    let json_p = JSON::init(context);

    context
        .register_global_property(js_string!("JSON"), json_p, Attribute::READONLY)
        .expect("the Person builtin shouldn't exist");

    context
        .register_global_class::<Person>()
        .expect("the Person builtin shouldn't exist");

    // ---------
    let person_class = context.get_global_class::<Person>();

    let person_constructor = match person_class {
        Some(val) => {
            let args: &[JsValue] = &[JsValue::from(js_string!("Jane")), JsValue::from(31)];
            let constructor = &val.constructor();
            constructor.construct(args, Some(constructor), context).map(JsValue::from)
        }
        None => Err(JsError::from_opaque(JsValue::from(js_string!("s"))))
    };
    let v = person_constructor.expect("it's initialized");
    context
        .register_global_property(js_string!("myPerson"), v, Attribute::READONLY)
        .expect("myPerson shouldn't exist");
    // ---------

    let navigator = Navigator::init(context);

    match navigator {
        Some(val) => {
            context
                .register_global_property(js_string!("navigator"), val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning navigator"),
    };

    let console = Console::init(context);
    context
        .register_global_property(js_string!(Console::NAME), console, Attribute::all())
        .expect("the console builtin shouldn't exist");

    /*let console = Console::init(context);

    context.register_global_property(key, value, attribute)
    match console {
        Some(val) => {
            context
                .register_global_property(js_string!("console"), val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning console"),
    };*/

    let clipboard = Clipboard::init(context);

    match clipboard {
        Some(val) => {
            context
                .register_global_property(js_string!("clipboard"), val, Attribute::READONLY)
                .expect("Registers");
        }
        None => println!("Error assigning clipboard"),
    };

    match doc {
        Some(document) => {
            let document_node = Node::init(context, &mut document.clone());

            match document_node {
                Some(val) => {
                    context
                        .register_global_property(js_string!("document"), val, Attribute::READONLY)
                        .expect("Registers");
                }
                None => println!("Error assigning document"),
            }
        }
        None => println!("Misc problem with the document"),
    };
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
    println!("init_browser");

    let res = context.eval(Source::from_bytes(js_data.as_str()));
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

    document::Document::new(html)
}

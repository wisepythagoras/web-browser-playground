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

use boa_engine::{
    object::{FunctionBuilder, ObjectInitializer},
    property::Attribute,
    Context, JsResult, JsString, JsValue,
};
use html::document;
use js::{clipboard::Clipboard, console::Console, navigator::Navigator, node::Node};
use std::{env, fs, process};

fn myfunction(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    match args.get(0) {
        Some(arg) => match arg.as_number() {
            Some(x) => println!("Argument to function: {}", x),
            None => println!("No return value."),
        },
        None => println!("No return value."),
    }
    match args.get(1) {
        Some(arg) => match arg.as_string() {
            Some(x) => println!("Argument to function: {}", x),
            None => println!("No return value."),
        },
        None => println!("No return value."),
    }

    args.get(0)
        .cloned()
        .unwrap_or_default()
        .pow(&JsValue::new(2), context)
}

fn init_browser(context: &mut Context, doc: Option<document::Document>) {
    let navigator = Navigator::init(context);

    match navigator {
        Some(val) => {
            context.register_global_property("navigator", val, Attribute::READONLY);
        }
        None => println!("Error assigning navigator"),
    };

    let console = Console::init(context);

    match console {
        Some(val) => {
            context.register_global_property("console", val, Attribute::READONLY);
        }
        None => println!("Error assigning console"),
    };

    let clipboard = Clipboard::init(context);

    match clipboard {
        Some(val) => {
            context.register_global_property("clipboard", val, Attribute::READONLY);
        }
        None => println!("Error assigning clipboard"),
    };

    match doc {
        Some(document) => {
            let document_node = Node::init(context, &mut document.clone());
        }
        None => {}
    };

    create_myfn(context);

    context.register_global_builtin_function("myfn", 1, myfunction);
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

    let res = context.eval(js_data);

    match res {
        Ok(_) => println!("Script was run"),
        Err(e) => {
            print!("Error: ");
            match e.as_string() {
                Some(e) => println!("{}", e.as_str()),
                None => println!("Unspecified error"),
            };
        }
    }
}

fn create_myfn(context: &mut Context) -> boa_engine::object::JsFunction {
    let function = FunctionBuilder::native(context, |_this, args, context| {
        match args.get(0) {
            Some(arg) => match arg.as_number() {
                Some(x) => println!("Argument to function: {}", x),
                None => println!("No return value."),
            },
            None => println!("No return value."),
        }

        args.get(0)
            .cloned()
            .unwrap_or_default()
            .pow(&JsValue::new(3), context)
    })
    .name("myfn2")
    .build();

    context.register_global_property(
        "myfn2",
        function.clone(),
        Attribute::READONLY | Attribute::NON_ENUMERABLE,
    );

    return function;
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

fn tests() {
    let script = r#"
	const test = (arg1) => {
	    if(arg1 != null) {
	        return myfn(arg1.x, "this is a test");
	    }
	    return 112233;
	};
	"#;

    let mut context = Context::default();

    // Populate the script definition to the context.
    let res = context.eval(script);
    let err: &JsString = &JsString::empty();

    match res {
        Ok(_) => println!("Script was loaded"),
        Err(e) => println!("{}", e.as_string().unwrap_or_else(|| err)),
    }

    // Create an object that can be used in eval calls.
    let arg = ObjectInitializer::new(&mut context)
        .property("x", 12, Attribute::READONLY)
        .build();
    context.register_global_property("arg", arg, Attribute::all());
    let navigator = Navigator::init(&mut context);

    match navigator {
        Some(val) => {
            context.register_global_property("navigator", val, Attribute::READONLY);
        }
        None => println!("Error assigning navigator"),
    }

    let function = FunctionBuilder::native(&mut context, |_this, args, context| {
        match args.get(0) {
            Some(arg) => match arg.as_number() {
                Some(x) => println!("Argument to function: {}", x),
                None => println!("No return value."),
            },
            None => println!("No return value."),
        }

        args.get(0)
            .cloned()
            .unwrap_or_default()
            .pow(&JsValue::new(3), context)
    })
    .name("myfn2")
    .build();
    context.register_global_property(
        "myfn2",
        function,
        Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE,
    );

    context.register_global_builtin_function("myfn", 1, myfunction);

    let value = context.eval("test(arg)").unwrap();

    match value.as_number() {
        Some(x) => println!("Result: {}", x),
        None => println!("No return value."),
    }

    assert_eq!(value.as_number(), Some(144.0));

    let value = context
        .eval("JSON.stringify(navigator.languages);")
        .unwrap();

    match value.as_string() {
        Some(x) => println!("Result: {}", x),
        None => println!("No return value."),
    }

    let value = context
        .eval("`${navigator.userAgent} - CPUs: ${navigator.hardwareConcurrency}`;")
        .unwrap();

    match value.as_string() {
        Some(x) => println!("Result: {}", x),
        None => println!("No return value."),
    }
}

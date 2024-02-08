use boa_engine::{
    js_string, object::ObjectInitializer, property::Attribute, symbol::JsSymbol, value::JsValue, Context, JsString, NativeFunction
};

use tap::{Conv, Pipe};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Console;

impl Console {
    const NAME: &'static str = "Console";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let s = JsSymbol::new(Some(JsString::from("Symbol.toStringTag")));
        // let to_string_tag = WellKnown::symbol_to_string_tag();
        let log_fn = Self::create_log_fn(context, "log");
        let warn_fn = Self::create_log_fn(context, "warn");
        let err_fn = Self::create_log_fn(context, "err");

        ObjectInitializer::new(context)
            .function(log_fn, js_string!("log"), 1)
            .function(warn_fn, js_string!("warn"), 1)
            .function(err_fn, js_string!("error"), 1)
            // .property("log", log_fn, attribute)
            // .property("warn", warn_fn, attribute)
            // .property("error", err_fn, attribute)
            // .property(to_string_tag, Self::NAME, attribute)
            // .property(JsSymbol::to_string_tag(), Self::NAME, attribute)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn create_log_fn(context: &mut Context, name: &str) -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], _: &mut Context| {
            let mut i = 0;

            while i < args.len() {
                Self::log_argument(args.get(i));
                i += 1;
                print!(" ");
            }

            println!("");

            Ok(JsValue::Undefined)
        };

        NativeFunction::from_fn_ptr(func)
    }

    fn log_argument(argument: Option<&JsValue>) {
        match argument {
            Some(arg) => {
                let a_type = arg.type_of();

                if a_type == "undefined" || a_type == "null" {
                    print!("{}", a_type);
                } else if a_type == "string" {
                    match arg.as_string() {
                        Some(v) => print!("{}", v.to_std_string_escaped()),
                        None => print!(""),
                    }
                } else if a_type == "number" {
                    match arg.as_number() {
                        Some(v) => print!("{}", v),
                        None => print!(""),
                    }
                } else if a_type == "object" {
                    print!("[object Object]");
                } else if a_type == "function" {
                    match arg.as_callable() {
                        Some(_) => print!("function () {} {}", '{', '}'),
                        None => {}
                    }
                } else {
                    print!("{} {:?}", a_type, arg);
                }
            }
            None => print!(""),
        }
    }
}

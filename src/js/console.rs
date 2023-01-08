use boa_engine::{
    context,
    object::{FunctionBuilder, JsFunction, ObjectInitializer},
    property::Attribute,
    symbol::WellKnownSymbols,
    value::JsValue,
    Context,
};

use tap::{Conv, Pipe};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Console;

impl Console {
    const NAME: &'static str = "Console";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let to_string_tag = WellKnownSymbols::to_string_tag();
        let log_fn = Self::create_log_fn(context);
        let warn_fn = Self::create_log_fn(context);
        let err_fn = Self::create_log_fn(context);

        ObjectInitializer::new(context)
            .property("log", log_fn, attribute)
            .property("warn", warn_fn, attribute)
            .property("error", err_fn, attribute)
            .property(to_string_tag, Self::NAME, attribute)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn create_log_fn(context: &mut Context) -> JsFunction {
        FunctionBuilder::native(context, |_this, args, context| {
            let mut i = 0;

            while i < args.len() {
                Self::log_argument(args.get(i));
                i += 1;
                print!(" ");
            }

            println!("");

            Ok(JsValue::Undefined)
        })
        .name("log")
        .build()
    }

    fn log_argument(argument: Option<&JsValue>) {
        match argument {
            Some(arg) => {
                let a_type = arg.type_of();

                if a_type == "undefined" || a_type == "null" {
                    print!("{}", a_type);
                } else if a_type == "string" {
                    match arg.as_string() {
                        Some(v) => print!("{}", v.as_str()),
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

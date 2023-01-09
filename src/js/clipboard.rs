use boa_engine::{
    builtins::promise::Promise,
    object::{FunctionBuilder, JsFunction, ObjectInitializer},
    property::Attribute,
    symbol::WellKnownSymbols,
    value::JsValue,
    Context, JsString,
};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::io::Read;
use wl_clipboard_rs::copy::{self, Options, Source};
use wl_clipboard_rs::paste::{self, get_contents, ClipboardType, Seat};

use tap::{Conv, Pipe};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Clipboard;

// Implementation of the Clibpboard API:
// https://developer.mozilla.org/en-US/docs/Web/API/Clipboard
impl Clipboard {
    const NAME: &'static str = "Clipboard";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let to_string_tag = WellKnownSymbols::to_string_tag();
        let write_text_fn = Self::write_text_fn(context);
        let read_text_fn = Self::read_text_fn(context);

        ObjectInitializer::new(context)
            .property("writeText", write_text_fn, attribute)
            .property("readText", read_text_fn, attribute)
            .property(to_string_tag, Self::NAME, attribute)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn write_text_fn(context: &mut Context) -> JsFunction {
        FunctionBuilder::native(context, |_this, args, context| {
            let p = context.intrinsics().constructors().promise().constructor();

            if args.len() < 1 {
                return Promise::reject(
                    &p.conv::<JsValue>(),
                    &[JsValue::from(JsString::new("No data to copy"))],
                    context,
                );
            }

            let arg = args.get(0);

            let data = match arg {
                Some(data) => match data.as_string() {
                    Some(v) => v,
                    None => "",
                },
                None => "",
            };

            let opts = Options::new();
            let res = opts.copy(
                Source::Bytes(data.to_string().into_bytes().into()),
                copy::MimeType::Autodetect,
            );

            match res {
                Ok(_) => Promise::resolve(&p.conv::<JsValue>(), &[JsValue::Undefined], context),
                Err(err1) => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    match ctx.set_contents(data.to_owned()) {
                        Ok(_) => {
                            Promise::resolve(&p.conv::<JsValue>(), &[JsValue::Undefined], context)
                        }
                        Err(err2) => {
                            println!("{}, {}", err1, err2);
                            let err_reason = JsValue::from(JsString::new("Unable to copy"));
                            Promise::reject(&p.conv::<JsValue>(), &[err_reason], context)
                        }
                    }
                }
            }
        })
        .name("writeText")
        .build()
    }

    fn read_text_fn(context: &mut Context) -> JsFunction {
        FunctionBuilder::native(context, |_this, _, context| {
            let p = context.intrinsics().constructors().promise().constructor();
            let res = get_contents(
                ClipboardType::Regular,
                Seat::Unspecified,
                paste::MimeType::Text,
            );

            let mut temp_str = String::new();
            match res {
                Ok((mut pipe, _)) => {
                    let mut contents = vec![];
                    let res = pipe.read_to_end(&mut contents);
                    match res {
                        Ok(_) => {
                            temp_str = String::from_utf8_lossy(&contents).to_string();
                            let clip_value = JsValue::from(JsString::new(temp_str.as_str()));
                            Promise::resolve(&p.conv::<JsValue>(), &[clip_value], context)
                        }
                        Err(_) => {
                            let err_reason = JsValue::from(JsString::new("Unable to read"));
                            Promise::reject(&p.conv::<JsValue>(), &[err_reason], context)
                        }
                    }
                }
                Err(err1) => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    match ctx.get_contents() {
                        Ok(val) => {
                            let clip_value = JsValue::from(JsString::new(val.as_str()));
                            Promise::resolve(&p.conv::<JsValue>(), &[clip_value], context)
                        }
                        Err(err2) => {
                            println!("{}, {}", err1, err2);
                            let err_reason = JsValue::from(JsString::new("Unable to read"));
                            Promise::reject(&p.conv::<JsValue>(), &[err_reason], context)
                        }
                    }
                }
            }
        })
        .name("readText")
        .build()
    }
}

use boa_engine::{
    js_string,
    // builtins::promise::Promise,
    object::{builtins::JsPromise, ObjectInitializer},
    property::Attribute,
    // symbol::WellKnownSymbols,
    value::JsValue,
    Context,
    JsError,
    JsString,
};
use boa_engine::{JsResult, NativeFunction};
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
        // let to_string_tag = WellKnownSymbols::to_string_tag();
        let write_text_fn = Self::write_text_fn(context);
        let read_text_fn = Self::read_text_fn(context);

        ObjectInitializer::new(context)
            // .property("writeText", write_text_fn, attribute)
            // .property("readText", read_text_fn, attribute)
            // .property(to_string_tag, Self::NAME, attribute)
            .function(write_text_fn, js_string!("writeText"), 1)
            .function(read_text_fn, js_string!("readText"), 1)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn promise_to_js_value(incomplete_promise: JsResult<JsPromise>) -> JsValue {
        match incomplete_promise {
            Ok(p) => match JsValue::try_from(p) {
                Ok(v) => v,
                Err(_) => JsValue::undefined(),
            },
            Err(_) => JsValue::undefined(),
        }
    }

    fn write_text_fn(ctx: &mut Context) -> NativeFunction {
        let func =
            |_this: &JsValue, args: &[JsValue], context: &mut Context| -> JsResult<JsValue> {
                // let p = context.intrinsics().constructors().promise().constructor();

                if args.len() < 1 {
                    // JsPromise::new(executor, context);
                    let rej_promise = JsPromise::reject(
                        JsError::from_opaque(JsValue::from(js_string!("No data to copy"))),
                        context,
                    );
                    let p = Self::promise_to_js_value(Ok(rej_promise));
                    return Ok(p);
                }

                let arg = args.get(0);

                let temp = JsString::from("");
                let data = match arg {
                    Some(data) => match data.as_string() {
                        Some(v) => v,
                        None => &temp,
                    },
                    None => &temp,
                };

                let opts = Options::new();
                let res = opts.copy(
                    Source::Bytes(data.to_std_string_escaped().into_bytes().into()),
                    copy::MimeType::Autodetect,
                );

                let res = match res {
                    Ok(_) => JsPromise::resolve(JsValue::Undefined, context),
                    Err(err1) => {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        match ctx.set_contents(data.to_std_string_escaped().to_owned()) {
                            Ok(_) => JsPromise::resolve(JsValue::Undefined, context),
                            Err(err2) => {
                                println!("{}, {}", err1, err2);
                                let err_reason = JsValue::from(JsString::from("Unable to copy"));
                                JsPromise::reject(JsError::from_opaque(err_reason), context)
                            }
                        }
                    }
                };

                Ok(Self::promise_to_js_value(Ok(res)))
            };

        NativeFunction::from_fn_ptr(func)
    }

    fn read_text_fn(context: &mut Context) -> NativeFunction {
        let func = |_this: &JsValue, _: &[JsValue], context: &mut Context| -> JsResult<JsValue> {
            let res = get_contents(
                ClipboardType::Regular,
                Seat::Unspecified,
                paste::MimeType::Text,
            );

            let temp_str;
            match res {
                Ok((mut pipe, _)) => {
                    let mut contents = vec![];
                    let res = pipe.read_to_end(&mut contents);
                    match res {
                        Ok(_) => {
                            temp_str = String::from_utf8_lossy(&contents).to_string();
                            let clip_value = JsValue::from(JsString::from(temp_str.as_str()));
                            let res_promise = JsPromise::resolve(
                                clip_value, context,
                            );
                            Ok(Self::promise_to_js_value(Ok(res_promise)))
                        }
                        Err(_) => {
                            let err_reason = JsValue::from(JsString::from("Unable to read"));
                            let rej_promise = JsPromise::reject(
                                JsError::from_opaque(err_reason),
                                context,
                            );
                            Ok(Self::promise_to_js_value(Ok(rej_promise)))
                        }
                    }
                }
                Err(err1) => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    match ctx.get_contents() {
                        Ok(val) => {
                            let clip_value = JsValue::from(JsString::from(val.as_str()));
                            let res_promise = JsPromise::resolve(
                                clip_value, context,
                            );
                            Ok(Self::promise_to_js_value(Ok(res_promise)))
                        }
                        Err(err2) => {
                            println!("{}, {}", err1, err2);
                            let err_reason = JsValue::from(JsString::from("Unable to read"));
                            let rej_promise = JsPromise::reject(
                                JsError::from_opaque(err_reason),
                                context,
                            );
                            Ok(Self::promise_to_js_value(Ok(rej_promise)))
                        }
                    }
                }
            }
        };

        NativeFunction::from_fn_ptr(func)
    }
}

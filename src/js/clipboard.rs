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
use wl_clipboard_rs::copy::{MimeType, Options, Source};

use tap::{Conv, Pipe};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Clipboard;

impl Clipboard {
    const NAME: &'static str = "Clipboard";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let to_string_tag = WellKnownSymbols::to_string_tag();
        let write_text_fn = Self::write_text_fn(context);

        ObjectInitializer::new(context)
            .property("writeText", write_text_fn, attribute)
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
                MimeType::Autodetect,
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
        .name("write")
        .build()
    }
}

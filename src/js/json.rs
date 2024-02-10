use std::hash::Hash;

use boa_engine::{
    js_string,
    object::ObjectInitializer,
    value::JsValue,
    Context,
    JsNativeError,
    JsObject,
    JsResult,
    JsString,
    NativeFunction
};
use serde_json::Value;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct JSON;

impl JSON {
    const NAME: &'static str = "JSON";

    pub(crate) fn init(context: &mut Context) -> JsObject {
        let parse_fn = Self::create_parse_fn();
        let stringify_fn = Self::create_stringify_fn();

        ObjectInitializer::new(context)
            .function(parse_fn, js_string!("parse"), 1)
            .function(stringify_fn, js_string!("stringify"), 1)
            .build()
    }

    fn create_parse_fn() -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], ctx: &mut Context| -> JsResult<JsValue> {
            let raw_val = args
                .get(0)
                .cloned()
                .unwrap_or_default();
            let val = raw_val
                .as_string()
                .expect("First argument is a string")
                .to_std_string_escaped();
            let val_str = val.as_str();
            let v: Value = serde_json::from_str(val_str).expect("parses");
            let ret_val = JsValue::from_json(&v, ctx);

            match ret_val {
                Ok(val) => Ok(val),
                Err(e) => {
                    Err(JsNativeError::typ()
                        .with_message(e.to_string())
                        .into())
                },
            }
        };

        NativeFunction::from_fn_ptr(func)
    }

    fn create_stringify_fn() -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], context: &mut Context| -> JsResult<JsValue> {
            let raw_val = args
                .get(0)
                .cloned()
                .unwrap_or_default();
            let json_val = raw_val.to_json(context);

            match json_val {
                Ok(val) => {
                    let stringified = serde_json::to_string(&val);

                    match stringified {
                        Ok(val) => {
                            Ok(JsValue::from(JsString::from(val)))
                        },
                        Err(e) => {
                            Err(JsNativeError::typ()
                                .with_message(e.to_string())
                                .into())
                        }
                    }
                }
                Err(e) => {
                    Err(JsNativeError::typ()
                        .with_message(e.to_string())
                        .into())
                }
            }
        };

        NativeFunction::from_fn_ptr(func)
    }
}

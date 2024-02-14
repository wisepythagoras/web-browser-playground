use boa_engine::{
    class::{Class, ClassBuilder}, error::JsNativeError, js_string, native_function::NativeFunction, property::Attribute, Context, JsArgs, JsData, JsObject, JsResult, JsString, JsValue, Source
};
use boa_gc::{Finalize, Trace};
use serde_json::Value;
use std::future::Future;
// use crate::js::utils;

// We create a new struct that is going to represent a person.
//
// We derive `Debug`, `Trace` and `Finalize`, it automatically implements `NativeObject`
// so we can pass it as an object in Javascript.
//
// The fields of the struct are not accessible by Javascript unless we create accessors for them.
/// Represents a `Person` object.
#[derive(Debug, Trace, Finalize, JsData)]
pub(crate) struct Response {
    status: u16,
    body: JsString,
    body_used: bool,
    ok: bool,
}

impl Response {
    fn json(this: &JsValue, _: &[JsValue], context: &mut Context) -> impl Future<Output = JsResult<JsValue>> {
        let object = this.as_object().expect("parses into an object");
        let response = object.downcast_ref::<Response>().expect("is a response object");
        let val = response.body
            .to_std_string()
            .expect("body is a string");

        // The lines below were meant to be in the async block, but Rust complains about the lifetime
        // of the context. So I moved it here so rust would stfu.
        let val_str = val.as_str();
        let v: Value = serde_json::from_str(val_str).expect("parses");

        let ret_val = JsValue::from_json(&v, context);

        async move {
            // TODO: Mark the body used here and prevent other instance function, such as `text` from
            // consuming the body.

            match ret_val {
                Ok(val) => Ok(val),
                Err(e) => {
                    Err(JsNativeError::typ()
                        .with_message(e.to_string())
                        .into())
                },
            }

            // Err(JsNativeError::typ()
            // .with_message("'this' is not a Person object")
            // .into())
        }
    }

    fn text(this: &JsValue, _: &[JsValue], _: &mut Context) -> impl Future<Output = JsResult<JsValue>> {
        let object = this.as_object().expect("parses into an object");
        let response = object.downcast_ref::<Response>().expect("is a response object");
        let val = response.body
            .to_std_string()
            .expect("body is a string");
        
        async move {
            let val = JsValue::from(JsString::from(val));
            Ok(val)
        }
    }

    fn clone(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Ok(JsValue::undefined())
    }

    fn blob(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Ok(JsValue::undefined())
    }

    fn array_buffer(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Ok(JsValue::undefined())
    }

    fn form_data(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        Ok(JsValue::undefined())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Response/Response
impl Class for Response {
    const NAME: &'static str = "Response";

    fn data_constructor(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        let body = args.get_or_undefined(0).to_string(context)?;
        let ok = true;
        let body_used = false;
        let status = 200;

        let response = Response { body, ok, body_used, status };

        Ok(response)
    }

    fn object_constructor(
        instance: &JsObject,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<()> {
        let body = args.get_or_undefined(0).to_string(context)?;

        instance.set(js_string!("body"), body, true, context)?;

        Ok(())
    }

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class.method(
            js_string!("json"),
            0,
            NativeFunction::from_async_fn(Self::json),
        );

        class.method(
            js_string!("text"),
            0,
            NativeFunction::from_async_fn(Self::text),
        );

        Ok(())
    }
}

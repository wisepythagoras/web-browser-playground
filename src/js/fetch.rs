use std::{borrow::Borrow, cell::RefCell, future::Future};
use boa_engine::{
    js_string, object::builtins::JsPromise, Context, JsError, JsNativeError, JsResult, JsString, JsValue
};
use crate::js::response::Response;
use smol::future;

use super::utils::js_promise_to_js_value;

enum RequestType {
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS,
}

pub(crate) fn fetch2(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let raw_url = args
        .get(0)
        .cloned()
        .unwrap_or_default();
    let url = raw_url
        .as_string()
        .expect("First argument is a string");
    let opts = args.get(1).cloned().unwrap_or_default();
    let mut method_type = RequestType::GET;
    let url_str = url.to_std_string_escaped();

    if opts.is_object() {
        let opts_obj = opts.as_object().expect("This is an object");
        
        if opts_obj.has_property(js_string!("method"), context).expect("works") {
            let method = opts_obj
                .get(js_string!("method"), context)
                .expect("Has the property");
            let method_str = method.as_string()
                .expect("")
                .to_std_string_escaped();

            method_type = match method_str.as_str() {
                "POST" => RequestType::POST,
                "DELETE" => RequestType::DELETE,
                "PUT" => RequestType::PUT,
                "OPTIONS" => RequestType::OPTIONS,
                _ => RequestType::GET,
            }
        }
    }

    let promise = JsPromise::new(|resolvers, context| {
        let response_class = context.get_global_class::<Response>()
            .expect("Response has been initialized");
        let constructor = &response_class.constructor();

        let ret_val = match method_type {
            RequestType::GET => {
                let resp = reqwest::blocking::get(url_str)
                    .expect("Success");
                let text = resp.text().expect("Converts to text");
                let str_resp = JsString::from(text.as_str());
                let args: &[JsValue] = &[JsValue::from(str_resp)];
                let obj = constructor.construct(args, Some(constructor), context)
                    .expect("constructs");

                JsValue::from(obj)
            }
            _ => {
                println!("This type of method is not yet implemented");
                JsValue::undefined()
            }
        };

        resolvers.resolve.call(&JsValue::undefined(), &[ret_val], context)?;

        Ok(JsValue::undefined())
    }, context);

    Ok(js_promise_to_js_value(promise))
}

// https://github.com/boa-dev/boa/blob/main/examples/src/bin/derive.rs
pub(crate) fn fetch_fn<'a>(
    _this: &JsValue,
    args: &'a [JsValue],
    context: &mut Context,
) -> impl Future<Output = JsResult<JsValue>> {
    let raw_url = args
        .get(0)
        .cloned()
        .unwrap_or_default();
    let url = raw_url
        .as_string()
        .expect("First argument is a string");
    let opts = args.get(1).cloned().unwrap_or_default();
    let mut method_type = RequestType::GET;
    let url_str = url.to_std_string_escaped();

    if opts.is_object() {
        let opts_obj = opts.as_object().expect("This is an object");
        
        if opts_obj.has_property(js_string!("method"), context).expect("works") {
            let method = opts_obj
                .get(js_string!("method"), context)
                .expect("Has the property");
            let method_str = method.as_string()
                .expect("")
                .to_std_string_escaped();

            method_type = match method_str.as_str() {
                "POST" => RequestType::POST,
                "DELETE" => RequestType::DELETE,
                "PUT" => RequestType::PUT,
                "OPTIONS" => RequestType::OPTIONS,
                _ => RequestType::GET,
            }
        }
    }

    // let response_class = context.get_global_class::<Response>()
    //     .expect("Response has been initialized");
    // let args: &[JsValue] = &[];
    // let constructor = &response_class.constructor();
    // let ctx = RefCell::new(context);

    async move {
        // let response_class = context.get_global_class::<Response>()
        //     .expect("Response has been initialized");
        // let constructor = &response_class.constructor();

        let ret_val = match method_type {
            RequestType::GET => {
                let resp = reqwest::blocking::get(url_str)
                    .expect("Success");

                println!("Status code: {}", resp.status().as_u16());
                // TODO: We should not automatically get the text value. Instead we want to
                // create an instance of the Response object (https://developer.mozilla.org/en-US/docs/Web/API/Response)
                let text = resp.text().expect("Converts to text");

                let str_resp = JsString::from(text.as_str());
                // obj.set(js_string!("body"), str_resp, true, context);
                JsValue::from(str_resp)

                // let args: &[JsValue] = &[JsValue::from(str_resp)];
                // let obj = constructor.construct(args, Some(constructor), context)
                //     .expect("constructs");

                // JsValue::from(obj)
            }
            _ => {
                println!("This type of method is not yet implemented");
                JsValue::undefined()
            }
        };

        Ok(ret_val)
    }
}

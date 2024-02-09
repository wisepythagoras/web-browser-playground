use std::future::Future;

use boa_engine::{js_string, Context, JsResult, JsString, JsValue};

enum RequestType {
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS,
}

// https://github.com/boa-dev/boa/blob/main/examples/src/bin/derive.rs
pub(crate) fn fetch_fn(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context
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
        println!("Opts is an object!");
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

    async move {
        let ret_val = match method_type {
            RequestType::GET => {
                let resp = reqwest::blocking::get(url_str)
                    .expect("Success");

                println!("Status code: {}", resp.status().as_u16());
                // TODO: We should not automatically get the text value. Instead we want to
                // create an instance of the Response object (https://developer.mozilla.org/en-US/docs/Web/API/Response)
                let text = resp.text().expect("Converts to text");

                let str_resp = JsString::from(text.as_str());
                JsValue::from(str_resp)
            }
            _ => {
                println!("This type of method is not yet implemented");
                JsValue::undefined()
            }
        };

        Ok(ret_val)

        // Ok(JsValue::undefined())
    }
}

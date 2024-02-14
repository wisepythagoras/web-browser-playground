use boa_engine::{object::builtins::JsPromise, JsResult, JsValue};

pub fn promise_to_js_value(incomplete_promise: JsResult<JsPromise>) -> JsValue {
    match incomplete_promise {
        Ok(p) => match JsValue::try_from(p) {
            Ok(v) => v,
            Err(_) => JsValue::undefined(),
        },
        Err(_) => JsValue::undefined(),
    }
}

pub fn js_promise_to_js_value(promise: JsPromise) -> JsValue {
    match JsValue::try_from(promise) {
        Ok(v) => v,
        Err(_) => JsValue::undefined(),
    }
}

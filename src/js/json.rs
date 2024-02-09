use boa_engine::{
    js_string, object::ObjectInitializer, property::Attribute, value::JsValue, Context, JsObject, JsString, NativeFunction
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct JSON;

impl JSON {
    const NAME: &'static str = "JSON";

    pub(crate) fn init(context: &mut Context) -> JsObject {
        let parse_fn = Self::create_parse_fn(context);
        let stringify_fn = Self::create_stringify_fn(context);

        ObjectInitializer::new(context)
            .function(parse_fn, js_string!("parse"), 1)
            .function(stringify_fn, js_string!("stringify"), 1)
            .build()
    }

    fn create_parse_fn(context: &mut Context) -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], ctx: &mut Context| {
            let raw_val = args
                .get(0)
                .cloned()
                .unwrap_or_default();
            let val = raw_val
                .as_string()
                .expect("First argument is a string")
                .to_std_string_escaped();
            let val_str = val.as_str();
            let mut obj_init = ObjectInitializer::new(ctx);

            let parse_result = json::parse(val_str);
            let ret_val = match parse_result {
                Ok(parsed) => {
                    let mut iter = parsed.entries();

                    while let Some((k, v)) = iter.next() {
                        let mut val = JsValue::undefined();

                        if v.is_number() {
                            val = JsValue::from(v.as_f64().expect("a number should be parsed"));
                        } else if v.is_string() {
                            val = JsValue::from(JsString::from(v.as_str().expect("a string should be parsed")));
                        } else if v.is_boolean() {
                            val = JsValue::from(v.as_bool().expect("a boolean is parsed"));
                        } else if v.is_null() {
                            val = JsValue::null();
                        }

                        obj_init.property(js_string!(k), val, Attribute::empty());
                    }

                    let obj = obj_init.build();
                    JsValue::from(obj)
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    JsValue::Undefined
                }
            };

            Ok(ret_val)
        };

        // NativeFunction::from_copy_closure(func)
        NativeFunction::from_fn_ptr(func)
    }

    fn create_stringify_fn(context: &mut Context) -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], _: &mut Context| {
            Ok(JsValue::Undefined)
        };

        NativeFunction::from_fn_ptr(func)
    }
}

use boa_engine::{
    js_string,
    object::ObjectInitializer,
    property::Attribute,
    value::JsValue,
    Context,
    JsObject,
    JsString,
    NativeFunction
};
use json::JsonValue;

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

    fn obj_to_js_object(val: JsonValue, context: &mut Context) -> JsObject {
        if val.is_object() {
            let mut iter = val.entries();
            let mut properties: Vec<(&str, JsValue)> = Vec::new();

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
                } else if v.is_object() {
                    val = JsValue::from(Self::obj_to_js_object(v.clone(), context));
                }

                properties.push((k, val));
            }

            let mut obj_init = ObjectInitializer::new(context);

            for (k, v) in properties {
                obj_init.property(js_string!(k), v, Attribute::empty());
            }

            let obj = obj_init.build();

            return obj;
        }

        JsObject::default()
    }

    fn create_parse_fn() -> NativeFunction {
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

            let parse_result = json::parse(val_str);
            let ret_val = match parse_result {
                Ok(parsed) => {
                    let obj = Self::obj_to_js_object(parsed, ctx);
                    JsValue::from(obj)
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    JsValue::Undefined
                }
            };

            Ok(ret_val)
        };

        NativeFunction::from_fn_ptr(func)
    }

    fn create_stringify_fn() -> NativeFunction {
        let func = |_this: &JsValue, args: &[JsValue], _: &mut Context| {
            Ok(JsValue::Undefined)
        };

        NativeFunction::from_fn_ptr(func)
    }
}

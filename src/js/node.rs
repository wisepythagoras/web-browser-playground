use crate::html::document::Document;
use boa_engine::{
    builtins::function::Function,
    object::{FunctionBuilder, JsFunction, ObjectInitializer},
    property::Attribute,
    symbol::WellKnownSymbols,
    value::JsValue,
    Context, JsResult,
};

use tap::{Conv, Pipe};

// #[derive(Clone, PartialEq, Eq)]
// pub(crate) struct Node {
//     document: Document,
// }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Node;

impl Node {
    const NAME: &'static str = "Node";

    pub(crate) fn init(context: &mut Context, doc: &mut Document) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let to_string_tag = WellKnownSymbols::to_string_tag();

        ObjectInitializer::new(context)
            .property(to_string_tag, Self::NAME, attribute)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn get_element_by_id_fn(context: &mut Context, doc: &mut Document) -> JsFunction {
        // Closures can only be coersed to fn types if the do not capture any variables
        // fn b(_this: &JsValue, args: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let b = |_this: &JsValue, args: &[JsValue], _: &mut Context| -> JsResult<JsValue> {
            if args.len() < 1 {
                // let cause = JsError::from_opaque("error!".into());
                return Ok(JsValue::Undefined);
            }

            // match doc.clone().get_element_by_id(String::from("")) {
            //     Some(a) => {}
            //     None => {}
            // };
            Ok(JsValue::Undefined)
        };

        FunctionBuilder::native(context, b)
            .name("getElementById")
            .build()
    }
}

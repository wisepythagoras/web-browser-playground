use std::cell::RefCell;

use crate::html::document::Document;
use boa_engine::{
    builtins::function::Function,
    object::{builtins::JsFunction, FunctionObjectBuilder, ObjectInitializer},
    property::Attribute,
    // symbol::WellKnownSymbols,
    value::JsValue,
    Context,
    JsResult,
    NativeFunction,
};

use boa_gc::{Finalize, GcRefCell, Trace};
use scraper::Html;
use tap::{Conv, Pipe};

// #[derive(Clone, PartialEq, Eq)]
// pub(crate) struct Node {
//     document: Document,
// }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Node;

impl Node {
    const NAME: &'static str = "Node";

    pub(crate) fn init(context: &mut Context, doc: Document) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        // let to_string_tag = WellKnownSymbols::to_string_tag();
        let get_id = Self::get_element_by_id_fn(context, doc);

        ObjectInitializer::new(context)
            // .property(to_string_tag, Self::NAME, attribute)
            .function(get_id, "getElementById", 1)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn get_element_by_id_fn(context: &mut Context, doc: Document) -> NativeFunction {
        /*
        let d = RefCell::new(doc);
        // let a = &doc;
        // let d = RefCell::new(m_test);
        // Closures can only be coersed to fn types if the do not capture any variables
        // fn b(_this: &JsValue, args: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        let b = move |_this: &JsValue, args: &[JsValue], _: &mut Context| -> JsResult<JsValue> {
            if args.len() < 1 {
                // let cause = JsError::from_opaque("error!".into());
                return Ok(JsValue::Undefined);
            }

            // match d.borrow_mut().get_element_by_id(String::from("")) {
            //     Some(a) => {}
            //     None => {}
            // };
            // test();
            Ok(JsValue::Undefined)
        };

        NativeFunction::from_copy_closure(b)
        */

        #[derive(Debug, Clone, Trace, Finalize)]
        struct TempDoc {
            pub contents: String,
        }

        let shit = TempDoc {
            contents: doc.contents,
        };

        NativeFunction::from_copy_closure_with_captures(
            |_, _, captures, context| {
                let mut captures = captures.borrow_mut();
                let TempDoc { contents } = &mut *captures;
                println!("{}", contents);
                // We obtain the `name` property of `captures.object`
                // let name = object.get("name", context)?;

                // // We create a new message from our captured variable.
                // let message = js_string!(
                //     utf16!("message from `"),
                //     &name.to_string(context)?,
                //     utf16!("`: "),
                //     greeting
                // );

                // We can also mutate the moved data inside the closure.
                // captures.greeting = js_string!(greeting, utf16!(" Hello!"));

                // println!("{}", message.to_std_string_escaped());
                // println!();

                // We convert `message` into `JsValue` to be able to return it.
                Ok(JsValue::Null)
            },
            // Here is where we move `clone_variable` into the closure.
            GcRefCell::new(shit),
        )
    }
}

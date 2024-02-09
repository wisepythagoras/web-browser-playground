extern crate libc;

use crate::html::document::Document;
use boa_engine::{
    js_string, object::ObjectInitializer, property::Attribute, value::JsValue, Context, JsError, JsNativeError, JsResult, JsString, NativeFunction
};
use std::{alloc::{alloc, dealloc, Layout}, mem};

use boa_gc::{Finalize, Trace};
use tap::{Conv, Pipe};

// #[derive(Clone, PartialEq, Eq)]
// pub(crate) struct Node {
//     document: Document,
// }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Node;

impl Node {
    const NAME: &'static str = "Node";

    pub(crate) fn init(context: &mut Context, doc: &Document) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        // let to_string_tag = WellKnownSymbols::to_string_tag();

        unsafe {
            /*let doc_raw_ptr: *mut Document =
                libc::malloc(mem::size_of::<Document>()) as *mut Document;

            if doc_raw_ptr.is_null() {
                panic!("failed to allocate memory");
            }

            *doc_raw_ptr = doc.clone();*/

            let layout = Layout::new::<Document>();
            let doc_raw_ptr = alloc(layout) as *mut Document;
            println!("is_null = {}, size = {}", doc_raw_ptr.is_null(), layout.size());

            *(doc_raw_ptr as *mut Document) = doc.clone();

            let get_id = Self::get_element_by_id_fn(context, doc_raw_ptr);

            ObjectInitializer::new(context)
                // .property(to_string_tag, Self::NAME, attribute)
                .function(get_id, js_string!("getElementById"), 1)
                .build()
                .conv::<JsValue>()
                .pipe(Some)
        }
    }

    fn get_element_by_id_fn(_: &mut Context, doc: *mut Document) -> NativeFunction {
        unsafe {
            let get_closure =
                move |_this: &JsValue, args: &[JsValue], _: &mut Context| -> JsResult<JsValue> {
                    if args.len() < 1 {
                        let error_str =
                        "Document.getElementById: At least 1 argument required, but only 0 passed";
                        let error: JsError = JsNativeError::typ().with_message(error_str).into();
                        return Err(error);
                    }

                    let arg = args.get(0);
                    let temp = JsString::from("");
                    let query_id = match arg {
                        Some(data) => match data.as_string() {
                            Some(v) => v,
                            None => &temp,
                        },
                        None => &temp,
                    };
                    println!("HERE!!!!");

                    match (*doc).get_element_by_id(query_id.to_std_string().unwrap()) {
                        Some(a) => {
                            println!("{:?}", a.value().classes);
                        }
                        None => {
                            println!("Not found");
                        }
                    };

                    Ok(JsValue::Undefined)
                };

            NativeFunction::from_copy_closure(get_closure)
        }
    }
}

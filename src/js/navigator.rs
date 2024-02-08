use boa_engine::{
    // builtins::,
    js_string, object::{builtins::JsArray, ObjectInitializer}, property::Attribute, value::JsValue, Context
    // JsResult,
};

use num_cpus;
use sys_locale::get_locale;
use tap::{Conv, Pipe};

static APP_NAME: &str = "Leebra";
static APP_ENGINE: &str = "LE";
static APP_VERSION: &str = "0.1.0";
static DEFAULT_LOCALE: &str = "en_US";
static LANGUAGES: &'static [&str] = &[DEFAULT_LOCALE, "en"];
static BUILD_ID: &str = "(none)";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Navigator;

impl Navigator {
    const NAME: &'static str = "Navigator";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        // let to_string_tag = WellKnownSymbols::to_string_tag();
        let locale = get_locale().unwrap_or_else(|| String::from(DEFAULT_LOCALE));
        let languages = JsArray::new(context);

        for val in LANGUAGES {
            languages.push(js_string!(*val), context).ok()?;
        }

        ObjectInitializer::new(context)
            .property(js_string!("appCodeName"), js_string!(APP_NAME), attribute)
            .property(js_string!("appName"), js_string!(APP_NAME), attribute)
            .property(js_string!("appVersion"), js_string!(APP_VERSION), attribute)
            .property(js_string!("cookieEnabled"), false, attribute)
            .property(js_string!("hardwareConcurrency"), num_cpus::get(), attribute)
            .property(js_string!("language"),js_string!(locale), attribute)
            .property(js_string!("languages"), languages, attribute)
            .property(js_string!("buildID"), js_string!(BUILD_ID), attribute)
            .property(js_string!("userAgent"), js_string!(Self::get_user_agent()), attribute)
            // .property(to_string_tag, Self::NAME, attribute)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    pub fn get_user_agent() -> String {
        let mut user_agent: String = String::new();
        let parts: &[&str] = &[APP_NAME, "/", APP_VERSION, " ", APP_ENGINE];

        for part in parts {
            user_agent.push_str(part)
        }

        user_agent
    }
}

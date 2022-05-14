use boa_engine::{
    object::ObjectInitializer,
    symbol::WellKnownSymbols,
    property::Attribute,
    value::JsValue,
    Context,
    JsResult,
};
use sys_locale::get_locale;
use tap::{Conv, Pipe};

static APP_NAME: &str = "Leebra";
static APP_ENGINE: &str = "LE";
static APP_VERSION: &str = "0.1.0";
static DEFAULT_LOCALE: &str = "en_US";
static BUILD_ID: &str = "(none)";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Navigator;

impl Navigator {
    const NAME: &'static str = "Navigator";

    pub(crate) fn init(context: &mut Context) -> Option<JsValue> {
        let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
        let to_string_tag = WellKnownSymbols::to_string_tag();
        let locale = get_locale().unwrap_or_else(|| String::from(DEFAULT_LOCALE));

        ObjectInitializer::new(context)
            .property("appCodeName", APP_NAME, attribute)
            .property("appName", APP_NAME, attribute)
            .property("appVersion", APP_VERSION, attribute)
            .property("language", locale, attribute)
            .property("buildID", BUILD_ID, attribute)
            .property("userAgent", Self::get_user_agent(), attribute)
            .property(to_string_tag, Self::NAME, attribute)
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

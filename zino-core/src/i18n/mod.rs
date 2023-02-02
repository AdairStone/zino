//! Internationalization and localization.

use crate::{BoxError, SharedString};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{collections::HashMap, env, fs, sync::LazyLock};
use unic_langid::{langid, LanguageIdentifier};

/// Translates the localization message.
pub fn translate(
    locale: &LanguageIdentifier,
    message: &str,
    args: Option<FluentArgs<'_>>,
) -> Result<SharedString, BoxError> {
    let bundle = LOCALIZATION
        .get(locale)
        .or_else(|| {
            let lang = locale.language;
            LOCALIZATION
                .iter()
                .find_map(|(locale, bundle)| (locale.language == lang).then_some(bundle))
        })
        .or_else(|| LOCALIZATION.get(&langid!("en-US")))
        .ok_or("the localization bundle does not exits")?;
    let pattern = bundle
        .get_message(message)
        .ok_or_else(|| format!("failed to get the localization message for `{message}`"))?
        .value()
        .ok_or_else(|| format!("failed to retrieve an option of the pattern for `{message}`"))?;

    let mut errors = vec![];
    match args {
        Some(args) => {
            let mut value = String::new();
            bundle.write_pattern(&mut value, pattern, Some(&args), &mut errors)?;
            if errors.is_empty() {
                Ok(value.into())
            } else {
                Err(format!("{errors:?}").into())
            }
        }
        None => {
            let value = bundle.format_pattern(pattern, None, &mut errors);
            if errors.is_empty() {
                Ok(value)
            } else {
                Err(format!("{errors:?}").into())
            }
        }
    }
}

/// Translation type.
type Translation = FluentBundle<FluentResource, IntlLangMemoizer>;

/// Localization.
static LOCALIZATION: LazyLock<HashMap<LanguageIdentifier, Translation>> = LazyLock::new(|| {
    let mut locales = HashMap::new();

    let project_dir = env::current_dir()
        .expect("the project directory does not exist or permissions are insufficient");
    let locale_dir = project_dir.join("./config/locale");
    match fs::read_dir(locale_dir) {
        Ok(entries) => {
            let files = entries.filter_map(|entry| entry.ok());
            for file in files {
                let locale_file = file.path();
                let ftl_string = fs::read_to_string(&locale_file).unwrap_or_else(|err| {
                    let locale_file = locale_file.to_string_lossy();
                    panic!("failed to read `{locale_file}`: {err}");
                });
                let resource =
                    FluentResource::try_new(ftl_string).expect("failed to parse an FTL string");
                if let Some(locale) = file
                    .file_name()
                    .to_str()
                    .map(|s| s.trim_end_matches(".ftl"))
                {
                    let lang = locale
                        .parse::<LanguageIdentifier>()
                        .unwrap_or_else(|_| panic!("failed to language identifier `{locale}`"));

                    let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
                    bundle
                        .add_resource(resource)
                        .expect("failed to add FTL resources to the bundle");
                    locales.insert(lang, bundle);
                }
            }
        }
        Err(err) => tracing::error!("{err}"),
    }

    locales
});
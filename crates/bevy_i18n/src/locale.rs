use std::{marker::PhantomData, path::PathBuf, str::FromStr};

use bevy_asset::{io::AssetSourceId, AssetPath};
use bevy_ecs::system::Resource;
use bevy_log::warn;

/// Stores as a ressource the current locale
/// for a given set of `Languages`. <br/>
/// Includes both a main and fallback [`Language`] for
/// redundancy. <br/> Unpon creation, the [`Locale`] will only
/// have a fallback [`Language`] defined as the default of `Languages`. <br/><br/>
/// Example usage of a [`Locale`]:
///```rs
///#[derive(Default)]
///pub enum MyLanguages {
///    #[default]
///    Français,
///    English,
///    Español,
///}
///
///fn main () {
///    App::new()
///        .init_resource::<Locale<MyLanguages>>()
///        ...
///}
///```
///
///The set of `Languages` will need to declare `impl From<MyLanguages> for Language`.
///Your implementation should not panic.<br/>
///If you have any doubt about your language codes please follow the
///[Unicode guidelines](https://cldr.unicode.org/index/cldr-spec/picking-the-right-language-code).
#[derive(Resource, Clone)]
pub struct Locale<Languages>
where
    Languages: Default,
    Language: From<Languages>,
{
    languages: PhantomData<Languages>,

    main: Option<Language>,
    fallback: Language,
}

impl<Languages> Default for Locale<Languages>
where
    Languages: Default,
    Language: From<Languages>,
{
    fn default() -> Self {
        Self {
            languages: PhantomData,
            main: None,
            fallback: Languages::default().into(),
        }
    }
}

/// Defines a single language to be used for localization.
#[derive(Clone, PartialEq, Eq)]
pub struct Language(unic_langid::LanguageIdentifier);

/// Describes an error that might happen while
/// creating the [`AssetPath`] for a localization folder.
#[derive(Debug)]
pub enum ToLocalePathError {
    /// The [`PathBuf`] is not valid UTF-8, might happen if the language code is not valid UTF-8.
    PathIsNonUTF8,

    /// Tried to make an [`AssetPath`] for the main language
    /// but no main language was defined.
    NoMainLanguague,
}

/// Describes an error that might happen while
/// creating a [`Language`].
#[derive(Debug)]
pub enum LanguageError {
    /// An unknown error occured while making the language
    Unknown,

    /// Not a valid ISO 639-3 language code
    InvalidLanguage,

    /// The provided subtag(s) is not valid
    InvalidSubtag,
}

impl<Languages> Locale<Languages>
where
    Languages: Default,
    Language: From<Languages>,
{
    /// Sets the main laguange of a locale
    pub fn set_main(&mut self, main: Languages) {
        self.main = Some(main.into());

        if &self.fallback == self.main.as_ref().unwrap() {
            warn!(
                target: "internationalization",
                warning = "Fallback language is the same as the main language.",
            );
        }
    }

    /// Sets the fallback laguange of a locale
    pub fn set_fallback(&mut self, fallback: Languages) {
        self.fallback = fallback.into();

        if &self.fallback == self.main.as_ref().unwrap() {
            warn!(
                target: "internationalization",
                warning = "Fallback language is the same as the main language.",
            );
        }
    }

    /// Returns an imutable reference of the current main language
    pub fn main(&self) -> &Option<Language> {
        &self.main
    }

    /// Returns an imutable reference of the current fallbakc language
    pub fn fallback(&self) -> &Language {
        &self.fallback
    }

    /// Adds the language's folder to the asset path. <br/>
    /// Example:
    /// - From
    /// `"assets/{my_asset}"`
    /// to
    /// `"assets/fr-CA/{my_asset}"`.
    ///
    /// Set `use_fallback` to true, to use the fallback language.
    pub fn to_locale_asset_path<'p>(
        &self,
        asset_path: impl Into<AssetPath<'p>>,
        use_fallback: bool,
    ) -> Result<AssetPath<'_>, ToLocalePathError> {
        let asset_path: AssetPath<'static> = asset_path.into().into_owned();

        let mut path = asset_path.path().to_owned();

        if use_fallback {
            path = PathBuf::from(self.fallback.get_name()).join(path);
        } else if let Some(main) = &self.main {
            path = PathBuf::from(main.get_name()).join(path);
        } else {
            return Err(ToLocalePathError::NoMainLanguague);
        }

        if let Some(path) = path.to_str() {
            let mut new_path = AssetPath::from(path.to_string());

            if let AssetSourceId::Name(name) = asset_path.source() {
                new_path = new_path.with_source(name.to_string());
            }

            if let Some(label) = asset_path.label() {
                new_path = new_path.with_label(label.to_owned());
            }

            return Ok(new_path);
        }
        Err(ToLocalePathError::PathIsNonUTF8)
    }
}

impl Language {
    /// Tries to create an new [`Language`] from an laguage code. <br/>
    /// See [`LanguageError`] for more details about potential errors.
    pub fn new(lang: &str) -> Result<Self, LanguageError> {
        match unic_langid::LanguageIdentifier::from_str(lang) {
            Ok(lang_id) => Ok(Self(lang_id)),
            Err(err) => Err(LanguageError::from(err)),
        }
    }

    /// Retreives the the name of the language (ie: it's language code)
    fn get_name(&self) -> String {
        format!("{}", self.0)
    }
}

impl From<unic_langid::LanguageIdentifierError> for LanguageError {
    fn from(err: unic_langid::LanguageIdentifierError) -> Self {
        match err {
            unic_langid::LanguageIdentifierError::Unknown => LanguageError::Unknown,
            unic_langid::LanguageIdentifierError::ParserError(err) => match err {
                unic_langid_impl::parser::errors::ParserError::InvalidLanguage => {
                    LanguageError::InvalidLanguage
                }
                unic_langid_impl::parser::errors::ParserError::InvalidSubtag => {
                    LanguageError::InvalidSubtag
                }
            },
        }
    }
}

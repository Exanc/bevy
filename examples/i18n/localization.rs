//! Example usage of a localization plugin using the `Localization` trait

use bevy::prelude::*;

use plugin::{MyLocalizationPlugin, TextLocalization};

#[derive(Default)]
pub enum MyLanguages {
    #[default]
    Français,
    English,
    Español,
}

impl From<MyLanguages> for Language {
    fn from(value: MyLanguages) -> Self {
        match value {
            MyLanguages::Français => Language::new("fr").unwrap(),
            MyLanguages::English => Language::new("en").unwrap(),
            MyLanguages::Español => Language::new("es").unwrap(),
        }
    }
}

fn main() {
    App::new()
        .init_resource::<Locale<MyLanguages>>()
        .add_plugins((
            DefaultPlugins,
            MyLocalizationPlugin::<MyLanguages>::new(),
            MyPlugin,
        ));
}

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Startup, create_ui);
    }
}

fn setup(mut locale: ResMut<Locale<MyLanguages>>) {
    locale.set_main(MyLanguages::Français);

    locale.set_fallback(MyLanguages::English);
}

fn create_ui(mut commands: Commands, localize: Res<TextLocalization<MyLanguages>>) {
    let i18n = localize.use_localization();

    commands
        .spawn(NodeBundle::default())
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: i18n("wellcome".to_string(), vec![]).into(),
                        style: TextStyle::default(),
                    }],
                    ..default()
                },
                ..default()
            });
        });
}

mod plugin {
    //! This module demonstrates an example implementation
    //! of the `Localization` trait

    use std::{collections::HashMap, fmt::Display, marker::PhantomData};

    use bevy::prelude::*;

    /// A protoype implementation of the localize trait
    #[derive(Resource)]
    pub struct TextLocalization<Languages>
    where
        Languages: Default,
        Language: From<Languages>,
    {
        /// This [PhantomData] is present to avoid warnings. <br/>
        /// The `Languages` generic type is used to grad the default [Language]
        /// of the user's language set.
        phantom: PhantomData<Languages>,

        main: Option<Language>,
        main_language_map: HashMap<String, String>,

        fallback: Language,
        fallback_language_map: HashMap<String, String>,
    }

    /// A enum representing errors thaht might happen while
    /// trying to retreive a text translation
    #[derive(Debug)]
    pub enum TextLocalizationError {
        /// The key was not associated to any ressource.
        RessourceNotFound,
    }

    /// The result of a text localization.
    #[derive(Debug)]
    pub enum TextLocalizationResult {
        /// The resulting translation
        Ok(String),

        /// The language, key, and error that occured while retreiving a ressource
        Err(String, String, TextLocalizationError),
    }

    /// A parameter for localization
    pub struct LocalizationParam {
        key: String,
        value: String,
    }

    pub struct MyLocalizationPlugin<Languages>(PhantomData<Languages>)
    where
        Languages: Default,
        Language: From<Languages>;

    impl<Languages> Plugin for MyLocalizationPlugin<Languages>
    where
        Languages: Default + Sync + Send + 'static,
        Language: From<Languages>,
    {
        fn build(&self, app: &mut App) {
            app.insert_resource(TextLocalization::<Languages>::init());
        }
    }

    impl<Languages> MyLocalizationPlugin<Languages>
    where
        Languages: Default,
        Language: From<Languages>,
    {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<Languages> TextLocalization<Languages>
    where
        Languages: Default,
        Language: From<Languages>,
    {
        fn init() -> Self {
            Self {
                phantom: PhantomData,
                main: None,
                main_language_map: HashMap::new(),
                fallback: Languages::default().into(),
                fallback_language_map: HashMap::new(),
            }
        }

        fn reload_translations(&mut self) {
            // Do something, load stuff

            unimplemented!()
        }
    }

    impl<Languages> Localization<String, Languages> for TextLocalization<Languages>
    where
        Languages: Default + Send + Sync + 'static, // Send, Sync & 'static may not be necessary for your implementation
        Language: From<Languages>,
    {
        type Key = String;

        type Argv = Vec<LocalizationParam>;

        type Result = TextLocalizationResult;

        fn set_locale(&mut self, locale: &Locale<Languages>) {
            self.main = locale.main().clone();

            self.fallback = locale.fallback().clone();

            self.reload_translations();
        }

        fn localize(&self, _: Self::Key, _: Self::Argv) -> Self::Result {
            unimplemented!()
        }
    }

    impl Display for TextLocalizationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TextLocalizationError::RessourceNotFound => f.write_str("Ressource not found"),
            }
        }
    }

    impl Into<String> for TextLocalizationResult {
        fn into(self) -> String {
            match self {
                TextLocalizationResult::Ok(res) => res,
                TextLocalizationResult::Err(lang, key, err) => {
                    warn!(
                        target: "TextLocalization",
                        warning = format!("An error occured while retreiving {lang}:{key}: {}", err),
                    );

                    format!("{lang}:{key}")
                }
            }
        }
    }
}

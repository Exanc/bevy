use std::fmt::Debug;

use bevy_ecs::system::Resource;

use crate::locale::{Language, Locale};

/// A common trait the for localization of ressources.
pub trait Localization<R, Languages>
where
    Languages: Default,
    Language: From<Languages>,
    Self: Resource,
{
    // Using associated types, since there should only be one way
    // of searching for the translation a an asset type

    /// Defines the type used to search for a specefic translation
    type Key: Eq + PartialEq;

    /// Defines the type used to insert data within a translation
    type Argv;

    /// Defines the type return by localization. <br/>
    /// Must either be of type `R` or implement `Into<R>` for
    /// the end-user's convenience.
    type Result: Into<R> + Debug;

    /// Notifies the [`Localization`] implementor that the locale has changed.
    fn set_locale(&mut self, locale: &Locale<Languages>);

    /// Given a key and arguments, localises an ressource.
    fn localize(&self, key: Self::Key, args: Self::Argv) -> Self::Result;

    /// Creates a new closure that can be used for localization in a short-hand form. <br/> <br/>
    /// Example usage:
    /// ```rs
    ///fn hello_world_system (
    ///    localizer: Res<MyLocalization>
    ///) {
    ///    let i18n = localizer.use_localization();
    ///
    ///    println!( "In your language we say hello like: {}", i18n("hello") );
    ///}
    ///```
    fn use_localization(&self) -> impl Fn(Self::Key, Self::Argv) -> Self::Result {
        |key: Self::Key, args: Self::Argv| self.localize(key, args)
    }
}

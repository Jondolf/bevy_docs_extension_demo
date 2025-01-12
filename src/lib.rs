//! A crate for testing [rustdoc] extensions and configurations for the [Bevy game engine].
//!
//! [rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
//! [Bevy game engine]: https://bevyengine.org

/// A re-export of Bevy.
pub use bevy;

use bevy::prelude::*;

/// An example of a [`Component`] type.
///
/// # Other Test Types
///
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
#[derive(Component)]
pub struct TestComponent;

/// An example of a [`Resource`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
#[derive(Resource)]
pub struct TestResource;

/// An example of an [`Event`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestPlugin`]
#[derive(Event)]
pub struct TestEvent;

/// An example of a [`Plugin`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestEvent`]
pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, _app: &mut App) {
        println!("Hello from `TestPlugin`!")
    }
}

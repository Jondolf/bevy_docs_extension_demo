//! A crate for testing [rustdoc] extensions and configurations for the [Bevy game engine].
//!
//! [rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
//! [Bevy game engine]: https://bevyengine.org

/// A re-export of Bevy.
pub use bevy;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// An example of a [`Component`] type.
///
/// # Other Test Types
///
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
#[derive(Component)]
pub struct TestComponent;

/// An example of a [`Resource`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
#[derive(Resource)]
pub struct TestResource;

/// An example of an [`Event`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
#[derive(Event)]
pub struct TestEvent;

/// An example of a [`Plugin`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPluginGroup`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, _app: &mut App) {
        println!("Hello from `TestPlugin`!")
    }
}

/// An example of a [`PluginGroup`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
pub struct TestPluginGroup;

impl PluginGroup for TestPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TestPlugin)
    }
}

/// An example of a [`SystemSet`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestAllTraits`]
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TestSystemSet {
    A,
    B,
}

/// An example of a type that implements all the relevant ECS traits.
///
/// **Note**: The tag for `Component` may be hidden because `Event` is also implemented.
/// This is done to avoid confusion, because every `Event` implements `Component`,
/// but they are typically not intended to be used as components.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
#[derive(Resource, Event, SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TestAllTraits;

impl Plugin for TestAllTraits {
    fn build(&self, _app: &mut App) {
        println!("Hello from `AllTraits`!")
    }
}

impl PluginGroup for TestAllTraits {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TestAllTraits)
    }
}

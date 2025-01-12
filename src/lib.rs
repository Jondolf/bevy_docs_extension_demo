//! A crate for testing [rustdoc] extensions and configurations for the [Bevy game engine].
//!
//! [rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
//! [Bevy game engine]: https://bevyengine.org

/// A re-export of Bevy.
pub use bevy;

use bevy::{
    app::PluginGroupBuilder,
    ecs::{schedule::ScheduleLabel, system::SystemParam},
    prelude::*,
};

/// An example of a [`Component`] type.
///
/// # Other Test Types
///
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(Component)]
pub struct TestComponent;

/// An example of a [`Resource`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(Resource)]
pub struct TestResource;

/// An example of an [`Asset`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(Asset, Reflect)]
pub struct TestAsset;

/// An example of an [`Event`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(Event)]
pub struct TestEvent;

/// An example of a [`Plugin`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
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
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
pub struct TestPluginGroup;

impl PluginGroup for TestPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TestPlugin)
    }
}

/// An example of a [`ScheduleLabel`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct TestScheduleLabel;

/// An example of a [`SystemSet`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemParam`]
/// - [`TestAllTraits`]
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TestSystemSet {
    A,
    B,
}

/// An example of a [`SystemParam`] type.
///
/// # Other Test Types
///
/// - [`TestComponent`]
/// - [`TestResource`]
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestAllTraits`]
#[derive(SystemParam)]
pub struct TestSystemParam;

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
/// - [`TestAsset`]
/// - [`TestEvent`]
/// - [`TestPlugin`]
/// - [`TestPluginGroup`]
/// - [`TestScheduleLabel`]
/// - [`TestSystemSet`]
/// - [`TestSystemParam`]
#[derive(
    Resource,
    Asset,
    Event,
    ScheduleLabel,
    SystemSet,
    SystemParam,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Reflect,
)]
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

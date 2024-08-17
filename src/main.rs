#![warn(
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::if_then_some_else_none,
    clippy::missing_const_for_fn,
    clippy::mixed_read_write_in_expression,
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::same_name_method,
    clippy::str_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::try_err,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::expect_used
)]
#![deny(
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::unimplemented,
    clippy::todo,
    clippy::dbg_macro,
    clippy::error_impl_error,
    clippy::exit,
    clippy::panic_in_result_fn,
    clippy::tests_outside_test_module
)]
#![allow(clippy::type_complexity, clippy::module_name_repetitions)]

mod assets;
mod cube;
mod interaction;
mod settings;

#[allow(unused_imports)]
mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_mod_picking::prelude::*;
    pub use bevy_tween::prelude::*;

    pub use super::MainState;
}
use prelude::*;

#[derive(States, Default, Clone, Hash, Eq, PartialEq, Debug)]
pub enum MainState {
    #[default]
    Loading,
    Playing,
}

fn main() {
    let mut app = App::new();
    #[cfg(feature = "release")]
    app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin {
        mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
    });

    app.add_plugins((DefaultPlugins, DefaultTweenPlugins, DefaultPickingPlugins));

    #[cfg(feature = "dev")]
    {
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }

    app.init_state::<MainState>();

    app.add_plugins((
        assets::AssetPlugin,
        cube::CubePlugin,
        interaction::InteractionPlugin,
    ));

    app.add_systems(OnEnter(MainState::Playing), setup_scene);

    app.run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            settings::CAMERA_LOCATION * 2.0,
            0.0, // settings::CAMERA_LOCATION,
            0.0, // settings::CAMERA_LOCATION,
        )
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(
            settings::LIGHT_LOCATION,
            settings::LIGHT_LOCATION,
            settings::LIGHT_LOCATION,
        ),
        ..default()
    });
}

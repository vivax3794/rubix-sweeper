use crate::prelude::*;
use bevy_asset_loader::prelude::*;


pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(crate::MainState::Loading)
                .continue_to_state(crate::MainState::Playing)
                
        );
    }
}



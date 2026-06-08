use bevy::prelude::*;

use super::character_creation::CharacterCreationPlugin;
use super::character_select::CharacterSelectPlugin;
use super::characters::{load_class_icons, CharacterRoster};
use super::title_menu::TitleMenuPlugin;
use super::world_select::WorldSelectPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterRoster>()
            .add_systems(Startup, load_class_icons)
            .add_plugins((
                TitleMenuPlugin,
                WorldSelectPlugin,
                CharacterSelectPlugin,
                CharacterCreationPlugin,
            ));
    }
}

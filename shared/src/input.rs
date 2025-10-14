//! Input Handling Systems
//!
//! Level cycling input handling for the tactical RPG.

use bevy::prelude::*;
use tracing::info;

use crate::level::LevelsResource;

/// System to handle left/right arrow key input for level cycling
pub fn level_cycling_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut levels_resource: ResMut<LevelsResource>,
) {
    let level_count = levels_resource.level_count();

    // Only process input if we have multiple levels
    if level_count <= 1 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        // Cycle to previous level (with wraparound)
        let new_index = if levels_resource.current_level_index == 0 {
            level_count - 1
        } else {
            levels_resource.current_level_index - 1
        };

        let old_level_name = levels_resource.current_level().name.clone();
        levels_resource.current_level_index = new_index;
        let new_level_name = &levels_resource.current_level().name;

        info!(
            "Level cycling: Previous (←) - switched from '{old_name}' to '{new_name}' (index {new_index})",
            old_name = old_level_name,
            new_name = new_level_name
        );
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        // Cycle to next level (with wraparound)
        let new_index = (levels_resource.current_level_index + 1) % level_count;

        let old_level_name = levels_resource.current_level().name.clone();
        levels_resource.current_level_index = new_index;
        let new_level_name = &levels_resource.current_level().name;

        info!(
            "Level cycling: Next (→) - switched from '{old_name}' to '{new_name}' (index {new_index})",
            old_name = old_level_name,
            new_name = new_level_name
        );
    }
}

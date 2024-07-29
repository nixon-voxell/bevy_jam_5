pub mod merchant;
pub mod tavern;

use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn layout_modal(commands: &mut Commands, m: impl FnOnce(&mut UiBuilder<Entity>)) {
    commands.ui_builder(UiRoot).column(|ui| {
        ui.style()
            .width(Val::Percent(100.))
            .height(Val::Percent(100.))
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .background_color(Color::BLACK.with_alpha(0.8));
        ui.row(|ui| {
            ui.style()
                .padding(UiRect::all(Val::Px(18.)))
                .border(UiRect::all(Val::Px(2.)))
                .border_color(Color::WHITE)
                .border_radius(BorderRadius::all(Val::Px(16.)))
                .background_color(Color::BLACK.with_alpha(0.8));
            m(ui)
        });
    });
}

use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};
use sickle_ui::ui_style::generated::{
    SetBackgroundColorExt, SetBorderRadiusExt, SetFlexGrowExt, SetFlexShrinkExt, SetFocusPolicyExt,
    SetFontColorExt, SetFontSizeExt, SetHeightExt, SetJustifyContentExt, SetJustifyItemsExt,
    SetJustifySelfExt, SetPaddingExt, SetWidthExt,
};
use sickle_ui::widgets::layout::column::UiColumnExt;
use sickle_ui::widgets::layout::container::UiContainerExt;
use sickle_ui::widgets::layout::label::{LabelConfig, UiLabelExt};
use sickle_ui::widgets::layout::row::UiRowExt;

use crate::game::actors::Structure;
use crate::ui::interaction::InteractionPalette;
use crate::ui::palette::{HEADER_SIZE, LABEL_SIZE};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Lost), show_lost_scren)
        .add_systems(
            PostUpdate,
            check_lost_status.run_if(in_state(Screen::Playing)),
        )
        .add_systems(Update, back_btn_interaction.run_if(in_state(Screen::Lost)));
}

#[derive(Component)]
struct ReturnToMenuButton;

fn show_lost_scren(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0))
                .focus_policy(FocusPolicy::Block)
                .background_color(Color::BLACK)
                .justify_self(JustifySelf::Center)
                .justify_content(JustifyContent::Center)
                .justify_items(JustifyItems::Center);

            ui.column(|_| {}).style().flex_grow(1.0);

            ui.column(|ui| {
                ui.row(|ui| {
                    ui.row(|_| {}).style().flex_grow(1.0);

                    ui.column(|ui| {
                        ui.label(LabelConfig::from("You lost!"))
                            .style()
                            .font_size(HEADER_SIZE);

                        ui.column(|_| {}).style().height(Val::Px(40.0));

                        ui.container(ButtonBundle::default(), |ui| {
                            ui.label(LabelConfig::from("Back to Main Menu"))
                                .style()
                                .font_size(LABEL_SIZE)
                                .font_color(Color::BLACK);
                        })
                        .insert((
                            InteractionPalette {
                                none: Color::WHITE,
                                hovered: Color::WHITE.darker(0.2),
                                pressed: Color::WHITE,
                            },
                            ReturnToMenuButton,
                        ))
                        .style()
                        .background_color(css::BLUE.into())
                        .border_radius(BorderRadius::all(Val::Px(12.0)))
                        .padding(UiRect::all(Val::Px(18.0)));
                    });

                    ui.row(|_| {}).style().flex_grow(1.0);
                });
            })
            .style()
            .flex_grow(0.0)
            .flex_shrink(1.0);

            ui.column(|_| {}).style().flex_grow(1.0);
        })
        .insert(StateScoped(Screen::Lost));
}

fn check_lost_status(
    q_structures: Query<(), With<Structure>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if q_structures.is_empty() {
        next_screen.set(Screen::Lost);
    }
}

fn back_btn_interaction(
    q_interactions: Query<&Interaction, (With<ReturnToMenuButton>, Changed<Interaction>)>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_screen.set(Screen::Title);
        }
    }
}

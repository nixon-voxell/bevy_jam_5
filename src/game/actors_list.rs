use super::actors::stats::ActorName;
use super::actors::PlayerActor;
use super::inventory::Inventory;
use super::selection::SelectedActor;
use super::INVENTORY_CAPACITY;
use crate::ui::palette::LABEL_SIZE;
use crate::ui::prelude::InteractionPalette;
use bevy::color::palettes::css;
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
pub struct ActorListContainer;

#[derive(Component)]
pub struct PlayerActorIcon;

pub fn actor_list_layout(ui: &mut UiBuilder<Entity>) {
    ui.row(|ui| {
        ui.insert(ActorListContainer)
            .style()
            .align_items(AlignItems::Start)
            .justify_content(JustifyContent::Start)
            .border(UiRect::all(Val::Px(2.)))
            .padding(UiRect::all(Val::Px(2.)))
            .column_gap(Val::Px(2.));
    });
}

#[derive(Resource, Default, Clone, Deref, DerefMut)]
pub struct PlayerActorList(pub Vec<Entity>);

#[derive(Component)]
pub struct SelectPlayerActorButton(pub Entity);

pub fn update_actor_list_container(
    mut local: Local<Vec<Entity>>,
    mut commands: Commands,
    container_query: Query<Entity, With<ActorListContainer>>,
    player_unit_list: Res<PlayerActorList>,
    selected_unit: Res<SelectedActor>,
) {
    if *local != player_unit_list.0 {
        local.clone_from(&player_unit_list.0);
    } else if !selected_unit.is_changed() {
        return;
    }

    for entity in container_query.iter() {
        commands.entity(entity).despawn_descendants();
        let mut ui = commands.ui_builder(entity);
        for unit_entity in player_unit_list.0.iter() {
            let back_color = match selected_unit.entity == Some(*unit_entity) {
                true => Color::WHITE,
                false => Color::BLACK,
            };

            ui.container(ButtonBundle::default(), |ui| {
                ui.style()
                    .border_color(css::DARK_GRAY.into())
                    .background_color(back_color)
                    .border(UiRect::all(Val::Px(2.)))
                    .column_gap(Val::Px(4.))
                    .padding(UiRect::all(Val::Px(4.)));
                ui.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::SpaceBetween);

                ui.icon("icons/warrior.png")
                    .insert(PlayerActorIcon)
                    .style()
                    .width(Val::Px(32.))
                    .height(Val::Px(32.));
            })
            .insert((
                InteractionPalette {
                    none: back_color,
                    hovered: css::DARK_RED.into(),
                    pressed: css::INDIAN_RED.into(),
                },
                SelectPlayerActorButton(*unit_entity),
            ));
        }
    }
}

#[derive(Component)]
pub struct SelectedActorNameLabel;

#[derive(Component)]
pub struct InventoryListLayout;

#[derive(Resource, Default)]
pub struct ItemSlotIcons(pub Vec<Entity>);

#[derive(Component)]
pub struct ItemSlotIndex(pub usize);

#[derive(Component)]
pub struct SellItemButton;

pub fn inventory_list_layout(ui: &mut UiBuilder<Entity>) -> Vec<Entity> {
    let mut out = vec![];
    ui.column(|ui| {
        ui.insert(InventoryListLayout);
        ui.style().align_items(AlignItems::End).row_gap(Val::Px(4.));
        ui.row(|ui| {
            ui.label(LabelConfig::from("unit name"))
                .insert(SelectedActorNameLabel);
        });
        ui.row(|ui| {
            ui.style().column_gap(Val::Px(2.));
            for i in 0..INVENTORY_CAPACITY {
                let id = ui
                    .container(ButtonBundle { ..default() }, |ui| {
                        ui.icon("").style().width(Val::Px(24.)).height(Val::Px(24.));
                    })
                    .insert(ItemSlotIndex(i))
                    .insert((InteractionPalette {
                        none: css::BLACK.into(),
                        hovered: css::DARK_GRAY.into(),
                        pressed: css::WHITE.into(),
                    },))
                    .style()
                    .border(UiRect::all(Val::Px(2.)))
                    .border_color(Color::WHITE)
                    .padding(UiRect::all(Val::Px(4.0)))
                    .border_radius(BorderRadius::all(Val::Px(4.0)))
                    .id();
                out.push(id);
            }
        });
        ui.row(|ui| {
            ui.row(|ui| {
                ui.style()
                    .position_type(PositionType::Absolute)
                    .margin(UiRect::top(Val::Percent(50.)));
                ui.container(ButtonBundle::default(), |ui| {
                    ui.label(LabelConfig::from("Sell item"))
                        .style()
                        .font_size(LABEL_SIZE);
                })
                .insert((
                    InteractionPalette {
                        none: css::RED.into(),
                        hovered: css::DARK_RED.into(),
                        pressed: css::INDIAN_RED.into(),
                    },
                    SellItemButton,
                ))
                .style()
                .padding(UiRect::all(Val::Px(10.)))
                .border_radius(BorderRadius::all(Val::Px(5.)));
            });
        });
    });
    out
}

pub fn update_selected_actor_name_label(
    selected_unit: Res<SelectedActor>,
    player_actor_list: Res<PlayerActorList>,
    name_query: Query<&ActorName>,
    mut label_query: Query<&mut Text, With<SelectedActorNameLabel>>,
) {
    let Some(entity) = selected_unit.entity else {
        return;
    };
    let Ok(name) = name_query.get(entity) else {
        return;
    };
    if selected_unit.is_changed() && player_actor_list.0.contains(&entity) {
        for mut text in label_query.iter_mut() {
            text.sections[0].value.clone_from(&name.0);
        }
    }
}

pub fn select_player_actor_btn_interaction(
    q_interactions: Query<(&Interaction, &SelectPlayerActorButton), Changed<Interaction>>,
    mut selected_actor: ResMut<SelectedActor>,
) {
    for (interaction, select) in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            selected_actor.entity = Some(select.0);
        }
    }
}

pub fn inventory_list_layout_vis(
    selected_actor: Res<SelectedActor>,
    mut query: Query<&mut Visibility, With<InventoryListLayout>>,
    pq: Query<&PlayerActor>,
) {
    if let Some(entity) = selected_actor.entity {
        if pq.contains(entity) {
            for mut v in query.iter_mut() {
                *v = Visibility::Visible;
            }
            return;
        }
    }

    for mut v in query.iter_mut() {
        *v = Visibility::Hidden;
    }
}

pub fn update_inventory_icons(
    mut commands: Commands,
    selected_unit: Res<SelectedActor>,
    iq: Query<&Inventory>,
    slot_icons: Res<ItemSlotIcons>,
    mut sq: Query<(&mut Style, &mut BorderColor, &Children)>,
) {
    let Some(selected_entity) = selected_unit.entity else {
        return;
    };

    let Ok(inventory) = iq.get(selected_entity) else {
        return;
    };

    for (i, e) in slot_icons.0.iter().enumerate() {
        if let Ok((mut style, mut b, children)) = sq.get_mut(*e) {
            style.display = if i < inventory.slot_count() {
                Display::Flex
            } else {
                Display::None
            };
            b.0 = if Some(i) == inventory.selected_item {
                css::YELLOW.into()
            } else {
                Color::BLACK.with_alpha(0.)
            };

            for c in children.iter() {
                if let Some(item) = inventory.get(i) {
                    commands
                        .ui_builder(*c)
                        .style()
                        .image(ImageSource::Path(format!("icons/{}.png", item.name)));
                } else {
                    commands
                        .ui_builder(*c)
                        .style()
                        .image(ImageSource::Path(String::new()));
                }
            }
        }
    }
}

pub fn select_item_btn_interaction(
    selected_unit: Res<SelectedActor>,
    q_interactions: Query<(&Interaction, &ItemSlotIndex), Changed<Interaction>>,
    mut iq: Query<&mut Inventory>,
) {
    let Some(entity) = selected_unit.entity else {
        return;
    };
    let Ok(mut inventory) = iq.get_mut(entity) else {
        return;
    };

    for (_, select) in q_interactions
        .iter()
        .filter(|(&i, _)| i == Interaction::Pressed)
    {
        if select.0 >= inventory.slot_count() {
            continue;
        }

        if inventory.get(select.0).is_some() {
            inventory.selected_item = Some(select.0);
        }
    }
}

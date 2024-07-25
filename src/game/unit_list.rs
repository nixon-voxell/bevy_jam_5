use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn unit_list_layout(mut commands: Commands) {
    commands.ui_builder(UiRoot)
    .row(|ui| {
        ui.column(|ui| {
            ui.style()
            .margin(UiRect::left(Val::Px(25.)).with_top(Val::Px(200.)))
            .align_items(AlignItems::Start)
            .justify_content(JustifyContent::Start)            
            .background_color(Color::BLACK)
            .border_color(Color::WHITE)
            .border(UiRect::all(Val::Px(2.)))
            .padding(UiRect::all(Val::Px(2.)))
            .row_gap(Val::Px(2.));

            for _ in 0..4 {
                    ui.row(|ui| {
                        ui.style()
                        .border_color(Color::WHITE)
                        .border(UiRect::all(Val::Px(2.)))
                        .column_gap(Val::Px(4.))
                        .padding(UiRect::all(Val::Px(4.)));
                        ui.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::SpaceBetween);
                        
                            ui.icon("tiles/human.png")
                            .style()
                            .width(Val::Px(64.))
                            .height(Val::Px(64.));

                            ui.row(|ui| {
                                ui.style().padding(UiRect::all(Val::Px(4.)));
                                ui.label(LabelConfig::from("3 / 3"))
                                .style()
                                .font_size(20.);
                            });
                        
                        
                    });
                
            }
        });
    });
}
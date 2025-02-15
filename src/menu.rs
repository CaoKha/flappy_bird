mod menu_state;

use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

use crate::game::game_state::GameState;
use crate::utils::despawn_screen;

#[derive(Component)]
struct OnMainMenuScreen;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(Update, close_menu.run_if(in_state(GameState::MainMenu)))
        .add_systems(
            OnExit(GameState::MainMenu),
            despawn_screen::<OnMainMenuScreen>,
        );
}

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Menu UI"),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                    ));
                });
        });
}


fn close_menu(mut next_state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyO) {
        next_state.set(GameState::InGame);
    }
}

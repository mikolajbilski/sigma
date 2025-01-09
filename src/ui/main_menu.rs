use bevy::{color::Color, prelude::*};

const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub(crate) fn spawn_menu(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let start_game_button = ButtonBundle {
        style: Style {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: BUTTON_COLOR.into(),
        ..default()
    };

    let start_game_text_node = TextBundle::from_section(
        "New Game",
        TextStyle {
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let container = commands.spawn(container_node).id();
    let button = commands.spawn(start_game_button).id();
    let start_game_text = commands.spawn(start_game_text_node).id();

    commands.entity(button).push_children(&[start_game_text]);
    commands.entity(container).push_children(&[button]);
}

pub(crate) fn main_menu_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, _, _, _) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("STARTING A GAME!");
            //TODO: Start a game
        }
    }
}

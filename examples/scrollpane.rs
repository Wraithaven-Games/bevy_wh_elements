use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_wh_elements::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(WhElementsPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut loader = AssetReference::new(&asset_server);
    ui().build(&mut commands, &mut loader);
}

fn ui() -> BoxedElement {
    WhCanvas::new() //
        .add_child(
            WhScreen::new(ScreenID(1)) //
                .bg_img("bg.png")
                .add_child(
                    WhScrollPane::new()
                        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                        .border(Color::WHITE, Val::Px(1.0))
                        .size(Val::Percent(50.0), Val::Percent(50.0))
                        .scroll_direction(ScrollDirection::Vertical)
                        .direction(ElementDirection::Column, Val::Px(0.0))
                        .add_children(vec![
                            list_elem("Apple"),
                            list_elem("Banana"),
                            list_elem("Cherry"),
                            list_elem("Date"),
                            list_elem("Orange"),
                            list_elem("Pear"),
                            list_elem("Pineapple"),
                            list_elem("Strawberry"),
                            list_elem("Watermelon"),
                            list_elem("Grape"),
                            list_elem("Kiwi"),
                            list_elem("Lemon"),
                            list_elem("Mango"),
                            list_elem("Peach"),
                            list_elem("Plum"),
                            list_elem("Raspberry"),
                            list_elem("Tomato"),
                            list_elem("Avocado"),
                            list_elem("Blackberry"),
                            list_elem("Blueberry"),
                            list_elem("Coconut"),
                            list_elem("Fig"),
                            list_elem("Grapefruit"),
                            list_elem("Guava"),
                            list_elem("Honeydew"),
                            list_elem("Kumquat"),
                        ]),
                ),
        )
}

const FRUIT_LIST: RadioButtonGroup = RadioButtonGroup(1);

fn list_elem(text: &str) -> BoxedElement {
    WhText::new()
        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
        .size(Val::Percent(100.0), Val::Px(30.0))
        .interaction(NodeInteraction::Radio(FRUIT_LIST))
        .change_border_on_active(Color::WHITE, Val::Px(1.0))
        .text(text)
}

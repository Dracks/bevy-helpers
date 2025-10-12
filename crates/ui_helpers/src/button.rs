use bevy::prelude::*;

#[derive(Component)]
pub struct Action<T: Message + Copy> {
    evt: T,
}

impl<T: Message + Copy> Action<T> {
    pub fn new(evt: T) -> Self {
        Self { evt }
    }
}

#[derive(Component)]
pub struct Hover<T: Message + Copy> {
    evt: T,
}

impl<T: Message + Copy> Hover<T> {
    pub fn new(evt: T) -> Self {
        Self { evt }
    }
}

pub fn button_press_system<T: Message + Copy>(
    buttons: Query<(&Interaction, &Action<T>), (Changed<Interaction>, With<Button>)>,
    mut action_evw: MessageWriter<T>,
) {
    for (interaction, action) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            action_evw.write(action.evt);
        }
    }
}

pub fn button_hover_system<T: Message + Copy>(
    buttons: Query<(&Interaction, &Hover<T>), (Changed<Interaction>, With<Button>)>,
    mut action_evw: MessageWriter<T>,
) {
    for (interaction, action) in buttons.iter() {
        if *interaction == Interaction::Hovered {
            action_evw.write(action.evt);
        }
    }
}

pub fn clean_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

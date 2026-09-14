use bevy::{prelude::*, ui::InteractionDisabled};

#[derive(Component, Clone)]
pub struct Action<T: Message + Clone> {
    evt: T,
}

impl<T: Message + Clone> Action<T> {
    pub fn new(evt: T) -> Self {
        Self { evt }
    }
}

impl<T: Default + Message + Clone> Default for Action<T> {
    fn default() -> Self {
        Action::new(T::default())
    }
}

#[derive(Component)]
pub struct Hover<T: Message + Clone> {
    evt: T,
}

impl<T: Message + Clone> Hover<T> {
    pub fn new(evt: T) -> Self {
        Self { evt }
    }
}

pub fn button_press_system<T: Message + Clone>(
    buttons: Query<
        (&Interaction, &Action<T>),
        (
            Changed<Interaction>,
            With<Button>,
            Without<InteractionDisabled>,
        ),
    >,
    mut action_evw: MessageWriter<T>,
) {
    for (interaction, action) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            action_evw.write(action.evt.clone());
        }
    }
}

pub fn button_hover_system<T: Message + Clone>(
    buttons: Query<
        (&Interaction, &Hover<T>),
        (
            Changed<Interaction>,
            With<Button>,
            Without<InteractionDisabled>,
        ),
    >,
    mut action_evw: MessageWriter<T>,
) {
    for (interaction, action) in buttons.iter() {
        if *interaction == Interaction::Hovered {
            action_evw.write(action.evt.clone());
        }
    }
}

pub fn clean_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

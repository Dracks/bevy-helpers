#[macro_export]
macro_rules! register_menu {
    (
        $fn_name:ident,
        $menu_state:expr,
        $action_type:ty,
        $spawn_system:expr,
        $actions_handler:expr
    ) => {
        pub fn $fn_name(app: &mut App) {
            app.add_systems(OnEnter($menu_state), $spawn_system)
                .add_message::<$action_type>()
                .add_systems(
                    Update,
                    ($actions_handler, button_press_system::<$action_type>),
                );
        }
    };
}

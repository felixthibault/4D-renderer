use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server:Res<AssetServer>) {
    let main_menu_entity: Entity= build_main_menu(&mut commands, &asset_server);
}
pub fn despawn_main_menu() {}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity: Entity = commands 
        .spawn(NodeBundle {
             
            background_color: Color::RED.into(),
            ..default()
        })
        .id();
    
    main_menu_entity
}
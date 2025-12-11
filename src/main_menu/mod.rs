mod components;
mod styles;
mod systems;

use systems::{layout::*,interactions::*};
use bevy::prelude::*;
use crate::AppState;

pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'main_menu' appelé, fonctionnel: Oui");
}
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app
        // OnEnter State Systems
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))

        // Systèmes
            //.add_systems((interact_with_play_button, interact_with_quit_button).in_set(OnUpdate(AppState::MainMenu)),)
        // OnExit State Systems
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Component)]
pub struct MyGameCamera;
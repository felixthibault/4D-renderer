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
#[derive(Plugins)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app
        // OnEnter State Systems
        .add_systems(Main, spawn_main_menu)

        // Systèmes
            //.add_systems((interact_with_play_button, interact_with_quit_button).in_set(OnUpdate(AppState::MainMenu)),)
        // OnExit State Systems
        .add_systems(Main, despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MyGameCamera;
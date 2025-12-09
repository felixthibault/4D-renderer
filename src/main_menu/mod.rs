mod components;
mod styles;
mod systems;

use systems::layout::*;
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
        .add_systems(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        // OnExit State Systems
        .add_systems(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
        ;
    }
}
use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
#[cfg(not(target_arch = "wasm32"))]
use bevy::{pbr::wireframe::{WireframeConfig, WireframePlugin},prelude::*};
use std::process::exit;
use std::*;

mod transformations;

#[path = "embarquation_b4d.rs"]
mod verifier_fichier;


fn main() {
    App::new()
        .add_plugins((#[cfg(not(target_arch = "wasm32"))] 
                        WireframePlugin{debug_flags:RenderDebugFlags},))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (#[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,setup
            ),)
        .run();
    println!("Entité triangle générée");
}
fn setup(){
    //Vérifier que le fichier binaire est présent sinon en créer un vide
    verifier_fichier();
    exit(0x0);
    //Vérifier que le fichier JSON est présent sinon en créer un avec des paramètres par défaut

}




#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
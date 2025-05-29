//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
#[cfg(not(target_arch = "wasm32"))]
use bevy::{pbr::wireframe::{WireframeConfig,},prelude::*};
use std::process::exit;
use std::*;

//mod transformations;
mod embarquation_b4d;
use crate::embarquation_b4d::*;
//use crate::transformations::*;
fn main() {
    App::new()
        .add_plugins((#[cfg(not(target_arch = "wasm32"))]
                    MinimalPlugins,))
        .add_systems(
            Startup, setup,)
        .add_systems(
            FixedUpdate,
            (#[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,))
        .run();
    println!("Entité triangle générée");
}
fn setup(){
    //Vérifier que le fichier JSON est présent sinon en créer un avec des paramètres par défaut
    
    //Vérifier que le fichier binaire est présent sinon en créer un vide
    verifier_fichier();
    exit(0x0);

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
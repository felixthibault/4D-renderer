#[cfg(not(target_arch = "wasm32"))]
use bevy::{pbr::wireframe::{WireframeConfig, WireframePlugin},prelude::*};


mod transformations;
mod embarquation_b4d;
use {std::*, transformations::*, embarquation_b4d::*};

fn main() {
    App::new()
        .add_plugins((#[cfg(not(target_arch = "wasm32"))] 
                        WireframePlugin,))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (#[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,
            ),)
        .run(setup);
    let carré=
    println!("Entité triangle générée");
}
fn setup(){
    //Vérifier que le fichier binaire est présent sinon en créer un vide
    let f= verifier_fichier()->std::io::Error;
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
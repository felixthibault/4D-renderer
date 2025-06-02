//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
#[cfg(not(target_arch = "wasm32"))]
use bevy::{pbr::wireframe::{WireframeConfig,},prelude::*};
use std::process::exit;
use std::*;
use tetra::window;

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
    exit(0x0);
}
fn setup(){
    //Vérifier que le fichier binaire est présent sinon en créer un vide
    //Vérifier que le fichier JSON est présent sinon en créer un avec des paramètres par défaut selon la version
    let VerificationFichier:String=verifier_fichier("Demarrage");
    
    //Vérification des paramètres de l'écran
    let ScreenSize:Vec(i32,i32)=[get_height(),get_width()];
    //Démarrage de l'interface
    print!("Interface démarrée");
    
    //Imprimer le débuggage
    if json.get("Debugging")==true{
    println!("Le fichier du projet a été ouvert {}",VerificationFichier);
    println!("Taille de l'écran de {}",ScreenSize);
    }
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


//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
//#[cfg(not(target_arch = "wasm32"))]
#![allow(unused)]
use bevy::{prelude::{Startup, App, DefaultPlugins, FixedUpdate, Component, Window, Query, With, Commands},
             window::PrimaryWindow};

use std::process::exit;
use num_traits::Zero;

mod objets;
mod transformations;
//mod embarquation_b4d;

use objets::*;
use transformations::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_systems(FixedUpdate, (update_objects, greet_objects))
        .add_systems(Startup, setup)
        .run();
    exit(0x0);
    

}

fn setup(query_window: Query<&Window, With<PrimaryWindow>>){
    objets::test();
    transformations::test();
    //embarquation_b4d::test();

    
    //Vérifier que le fichier binaire est présent sinon en créer un vide
    //Vérifier que le fichier JSON est présent sinon en créer un avec des paramètres par défaut selon la version
    //let verification_fichier:bool=verifier_fichier("Demarrage");
    
    //Vérification des paramètres de l'écran
    let (width,height):(u16,u16)=get_size(query_window);
    let _screen_size: (u16, u16)=(width,height);
    
    //Démarrage de l'interface
    print!("Démarrage de l'interface...\n Interface démarré.\n");
    print_window_size_system(query_window);
    //Imprimer le débuggage
    //if json.get("Debugging")==true{ println!("Le fichier du projet a été ouvert {}",verification_fichier); println!("Taille de l'écran de {}",screen_size); }
    println!("Débuggage du système:\nTaille de l'écran=({width}, {height}),\nApplication fonctionnelle: Oui,\nDémarage du renderer\n BIENVENUE AU RENDERER 3D ET 4D!!!!!");
    let in_progress:bool=true;
    match in_progress{
        true => print("CAD toujours en progrès."),
        false => print("CAD prêt à être utilisé."),
        _ => panik(),
    }
    //Tester si on peut générer une entité
    Entite::create_square(4f32);//Longueur de chaque côté=4

    exit_(0x0);
}

pub fn report_error(message:&str,code:&str){
    //Afficher fenêtre contenant une erreur mineure
    //Pour l'instant:
    println!("{} {}.",message,code);
}

fn get_size<R:From<u16>+Zero>(query_window: Query<&Window, With<PrimaryWindow>>)->(R,R){
    if let Ok(window)=query_window.single(){
        let x = window.physical_width();
        let y = window.physical_height();
        ((x as u16).into(),(y as u16).into())
    }
    else{(R::zero(),R::zero())}
}

fn print_window_size_system(query_window: Query<&Window, With<PrimaryWindow>>) {
    //Généré par AI de google search
    if let Ok(window) = query_window.single() {
        let physical_width = window.physical_width();
        let physical_height = window.physical_height();
        let logical_width = window.resolution.width();
        let logical_height = window.resolution.height();
        let scale_factor = window.resolution.scale_factor();

        println!("Physical Window Size: {}x{}", physical_width, physical_height);
        println!("Logical Window Size: {}x{}", logical_width, logical_height);
        println!("Scale Factor: {}", scale_factor);
    }
}

fn exit_(code:i32){
    print("Exiting.");
    exit(code);
}

fn update_objects(_:MesStructures){
    unsafe{do_nothing(())};
}

fn greet_objects(_:MesStructures){
    unsafe{do_nothing(())};
}


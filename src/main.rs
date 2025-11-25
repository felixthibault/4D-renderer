
//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
//#[cfg(not(target_arch = "wasm32"))]
#![allow(unused)]
use bevy::{prelude::{Startup, App, DefaultPlugins, FixedUpdate, Component, Window, Query, With, Commands},
             window::PrimaryWindow};

use std::process::exit;
use num_traits::Zero;

//mod objets;
mod transformations;
mod objets;
mod embarquation_b4d;

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
    embarquation_b4d::test();

    //Vérifier que le fichier JSON est présent sinon
    // en créer un avec des paramètres par défaut selon la version
    let json: serde_json::Value=embarquation_b4d::verifier_json();
    //Vérifier que le fichier binaire est présent sinon
    // en créer un vide selon les paramètres du fichier json
    embarquation_b4d::verifier_fichier(json);
    //Vérification des paramètres de l'écran    
    let (width,height):(u16,u16)=get_size(query_window);
    let _screen_size: (u16, u16)=(width,height);
    
    //Démarrage de l'interface
    print!("Démarrage de l'interface...\n Interface démarré.\n");
//print_window_size_system(query_window);
    //Imprimer le débogage
    let json_data:embarquation_b4d::Configuration=serde_json::from_slice(json.get("Configuration")
        .expect("Le fichier JSON devrait avoir l'index 'Configuration' "));
    let in_progress=json.get("Debugging")
        .expect("Le fichier JSON devrait avoir l'index 'Debugging' ")
        .to_bool();
    println!("Débogage du système:\nTaille de l'écran=({width}, {height}),
    \nApplication fonctionnelle: Oui,\nDémarage du renderer\n BIENVENUE AU RENDERER 3D ET 4D!!!!!");
    
    match in_progress{
        true => print("CAD toujours en progrès."),
        false => print("CAD prêt à être utilisé."),
        _ => panik(),
    }
    //Tester si on peut générer une entité simple
    //let test1=Point::new("point 1", Position::new(1,2,3));
    //Teste de création d'une super-rtucture (un polygone formé de lignes et de points)
    //let test2=Polygone::create_square(4.4f32);
    //dbg!(test1);
    //dbg!(test2);

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


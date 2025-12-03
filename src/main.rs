
//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
//#[cfg(not(target_arch = "wasm32"))]
#![allow(unused)]
use bevy::{prelude::{Startup, App, DefaultPlugins, FixedUpdate, Component, Window, Query, With, Commands},
             window::PrimaryWindow};

use std::process::exit;
use num_traits::Zero;

//mod objets;
mod transformations;

mod embarquation_b4d;
pub(crate) mod objets;
use objets::*;
use transformations::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        //.add_systems(FixedUpdate, (update_liste, update_cad ))
        .run();
    exit(0x0);
    

}

//-----Systems--------

fn setup(query_window: Query<&Window, With<PrimaryWindow>>){
    //Setuper la fonctionnalité du système
    objets::test();
    transformations::test();
    embarquation_b4d::test();
    winsdl::test();

    //Vérifier que le fichier JSON est présent sinon
    // en créer un avec des paramètres par défaut selon la version
    let json: embarquation_b4d::Configuration=embarquation_b4d::verifier_json(); // Ceci retourne la structure
        //du fichier json. On pourra plus facilement accéder aux items et index.

    //Vérifier que le fichier binaire est présent sinon
    // en créer un vide selon les paramètres du fichier json
    embarquation_b4d::verifier_fichier(&json);//Note à moi-même: il faut envoyer une référence de json
    // car ce type de structure n'implémente pas Copy, donc on ne peut envoyer la valeur sans aussi 
    // envoyer la variable au complet, et elle n'est pas retournée ensuite.

    //Vérification des paramètres de l'écran    
    let (width,height):(u16,u16)=get_size(query_window);
    let _screen_size: (u16, u16)=(width,height);
    
    //Démarrage de l'interface
    print!("Démarrage de l'interface...\n Interface démarré.\n");

    //Imprimer le débogage
    let (testing,in_progress)=(json.mathing.testing,json.mathing.debugging);
    println!("Débogage du système:\nTaille de l'écran=({width}, {height})");
    
    match in_progress{ //Si le projet est toujours en construction
        true => print("CAD toujours en progrès."),
        false => print("CAD prêt à être utilisé."),
        _ => panik(),
    }

    if testing{
        print_window_size_system(query_window);
        //Tester si on peut générer une entité simple
        let test1=Point::new(Position::new(1,2,3));
        //Teste de création d'une super-rtucture (un polygone formé de lignes et de points)
        let test2=Polygone::create_square(4.4f32);
        dbg!(test1);
        dbg!(test2);
        
        //Fin des tests
    }
    

    print("Application fonctionnelle: Oui,\nDémarage du renderer\n BIENVENUE AU RENDERER 3D ET 4D!!!!!");
    //Lancer l'interface avec le module winsdl et afficher les premières entités visibles selon le plan de vue

    //Chercher une bonne configuration -> Prendre les nombres du fichier de settings


    exit_(0x0);//Pour l'instant
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
    print("Exiting. Debug was successful.");
    exit(code);
}

fn update_objects(_:){
    unsafe{do_nothing(())};
}

fn greet_objects(_:){
    unsafe{do_nothing(())};
}

fn hello_world() {
    println!("hello world!");
}

fn println(msg:&str){
    println!("{}",msg);
    //Exemple d'utilisation:
    //print("Oh my glob!");
    //=> Oh my glob!
    //=>
}
fn print(msg:&str){
    print!("{}",msg);
    //Exemple d'utilisation:
    //print("Oh my glob!");
    //=> Oh my glob!
}
pub fn report_error(message:&str,code:&str){
    //Afficher fenêtre contenant une erreur mineure
    //Pour l'instant:
    println!("{} {}.",message,code);
}
#[allow(unconditional_recursion)]
pub fn unreachable(){
    type Unreachable=();
    let _x:Unreachable=unreachable();
    unreachable!()//Ceci n'est pas ateignable
}
pub fn not_implemented(){
    todo!();//This will panic
}
pub fn panik() {
    println("crash ans burn");
    panic!("crash and burn");
}
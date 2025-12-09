
//use bevy::render::{renderer::RenderAdapter, RenderDebugFlags};
//#[cfg(not(target_arch = "wasm32"))]
#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod renderer;
mod main_menu;
mod num_core;
mod systems;

use systems::*;
use crate::renderer::{embarquation_b4d, transformations, objets::*, winsdl};
use main_menu::MainMenuPlugin;
use renderer::RendererPlugin;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(RendererPlugin)
        // Startup Systems
        //.add_startup_system(spawn_camera)
        .add_systems(Startup, setup)
        // Systems
        .add_systems(transition_to_cad_state)
        .add_systems(transition_to_main_menu_state)
        .add_systems(exit_cad)
        //.add_systems(FixedUpdate, (update_liste, update_cad )
        .run();
}

//-----Systems--------

fn setup(query_window: Query<&Window, With<PrimaryWindow>>){
    //Setuper la fonctionnalité du système
    renderer::test();
    num_core::test();
    main_menu::test();
    systems::test();

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


fn update_objects(_:){
    unsafe{do_nothing(())};
}

fn greet_objects(_:){
    unsafe{do_nothing(())};
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Cad,
    //GameOver, //Voir ce que je pourrais mettre à la place
}
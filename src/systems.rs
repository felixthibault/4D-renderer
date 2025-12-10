//Inspiration de certains systèmes par https://github.com/frederickjjoubert/bevy-ball-game/blob/Episode-10/src/systems.rs
//dans la vidéo Learn Bevy 0.10 - EP10 - Bevy UI : 

//Référence pour le clavier: https://bevy-cheatbook.github.io/input/keyboard.html
use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};

use std::process::exit;
use num_traits::Zero;
use crate::AppState;

pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'systems' appelé, fonctionnel: Oui");
}

pub fn exit_cad(mut app_exit_event_writer: EventWriter<AppExit>){
    app_exit_event_writer.write(AppExit::Success);
}
pub fn exit_(code:i32){
    print("Exiting. Debug was successful.");
    exit(code);
}

pub unsafe fn do_nothing(_:()) -> (){
    return ()//Fonction doit être appelé avec unsafe{do_nothing};
}


pub fn get_size<R:From<u16>+Zero>(query_window: Query<&Window, With<PrimaryWindow>>)->(R,R){
    if let Ok(window)=query_window.single(){
        let x = window.physical_width();
        let y = window.physical_height();
        ((x as u16).into(),(y as u16).into())
    }
    else{(R::zero(),R::zero())}
}

pub fn print_window_size_system(query_window: Query<&Window, With<PrimaryWindow>>) {
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

pub fn hello_world() {
    println!("hello world!");
}

pub fn println(msg:&str){
    println!("{}",msg);
    //Exemple d'utilisation:
    //print("Oh my glob!");
    //=> Oh my glob!
    //=>
}
pub fn print(msg:&str){
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

fn get_a_job(){
    unsafe{do_nothing(())};
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

//Systèmes de transition

pub fn transition_to_cad_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Cad {
            app_state_next_state.set(AppState::Cad);
            println!("Entered AppState::Cad");
        }
    }
}

pub fn transition_to_main_menu_state(
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if app_state.0 != AppState::MainMenu {
        app_state_next_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}
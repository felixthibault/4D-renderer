#[allow(unused_imports)]
#[allow(unused_variables)]
use std::{*, fs::File,io::*, io::prelude::*};
use serde::Deserialize;

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier(){
    let  fichier= File::open("Préférences.json").expect("Le fichier de préférences devrait être read-only.");
    let json: serde_json::Value = serde_json::from_reader(fichier).expect("Le fichier de préférences devrait être de format JSON");
    //Utiliser json comme structure pour accéder au fichier JSON.
    
    let preference_ouverture = json.get("Ouverture_sécuritaire").unwrap();
    let nom_par_défaut:String=
        if preference_ouverture==true{json.get("Default_project_name").expect("Le nom par défaut du projet devrait exister
        si l'ouverture sécuritaire est activée.").to_string()}
        else{json.get("Preferred_default_project_name").expect("Le nom du par défaut de préférence devrait exister
        si l'ouverture sécuritaire est désactivée.").to_string()};

    let ouverture_terrain_result = File::open(&nom_par_défaut).unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            create_fichier(nom_par_défaut)
        }
        else {
            panic!("Problem opening the default file: {error:?}");
        }
    });
    print!("Fichier {ouverture_terrain_result:?} ouvert");
    
}
 
fn create_fichier(nom_fichier:String)-> File{
    let mut create_fichier=File::create(&nom_fichier).expect("Problem creating the file:");
    let message_de_création:String=format!("Voici le CAD du projet {}. Bonne chance! \n Ce document n'est pas conçu pour du DOS.",nom_fichier);
    create_fichier.write_all(message_de_création.as_bytes()).expect("Impossible d'écrire du texte au nouveau projet.");
    return create_fichier;
}

fn écrire_texte_fichier(mut projet:File,texte:String){
    projet.write(texte.as_bytes());
}
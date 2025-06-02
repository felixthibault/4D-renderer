#[allow(unused_imports)]
#[allow(unused_variables)]
use std::{*, fs::File,io::*, io::prelude::*};
use serde::Deserialize;

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier(TypeDeFichier:String){
    if TypeDeFichier=="Demarrage"{
        //Vérification des fichiers de démarrage standarts.
        let fichier= File::open("Préférences.json").expect("Le fichier de préférences devrait être read-only.");
        let json: serde_json::Value = serde_json::from_reader(fichier).expect("Le fichier de préférences devrait être de format JSON");
        //Utiliser json comme structure pour accéder au fichier JSON.
        
        let preference_ouverture = json.get("Ouverture_sécuritaire").unwrap();
        let nom_par_defaut:String=
            if preference_ouverture==true{json.get("Default_project_name").expect("Le nom par défaut du projet devrait exister
            si l'ouverture sécuritaire est activée.").to_string()}
            else{json.get("Preferred_default_project_name").expect("Le nom du par défaut de préférence devrait exister
            si l'ouverture sécuritaire est désactivée.").to_string()};
        let nom_fichier:String=nom_par_defaut;
    }
    else {
        //Ouverture du fichier spécifique
        let nom_fichier:String=TypeDeFichier;
    }
    let ouverture_terrain_result = File::open(&nom_fichier).unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            print!("Fichier du projet par défaut non trouvé, écriture d'un nouveau.");
            create_fichier(nom_fichier)
        }
        else {
            panic!("Problem opening the default file: {error:?}");
        }
    });
    print!("Fichier {ouverture_terrain_result:?} ouvert");
}
 
fn create_fichier(nom_fichier:String)-> File{
    let mut create_fichier=File::create(&nom_fichier).expect("Problem creating the file:");
    let message_de_creation:String=format!("Voici le CAD du projet {}. Bonne chance! \n Ce document n'est pas conçu pour du DOS.",nom_fichier);
    create_fichier.write_all(message_de_creation.as_bytes()).expect("Impossible d'écrire du texte au nouveau projet.");
    return create_fichier;
}

fn ecrire_texte_fichier(mut projet:File,texte:String)->std::io::Result<()>{
    projet.write(texte.as_bytes())?;
    //write!("Some bytes were written to {}",projet);
    Ok(())
}

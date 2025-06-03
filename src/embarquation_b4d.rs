#[allow(unused_imports)]
#[allow(unused_variables)]
use std::{*, fs::File,io::*, io::prelude::*};
use serde::Deserialize;

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier(TypeDeFichier:String)->String {
    if TypeDeFichier=="Demarrage"{
        //Vérification des fichiers de démarrage standarts.
        let parametre= File::open("Préférences.json").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            print!("Fichier des paramètres non trouvé, écriture d'un nouveau à partir de la version actuelle .");
            create_json("Préférences.json")
        }
        else {
            panic!("Le fichier de préférences devrait être read-only.");
        }
        let json: serde_json::Value = serde_json::from_reader(parametre).expect("Le fichier de préférences devrait être de format JSON");
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
            print!("Fichier du projet par défaut non trouvé, écriture d'un nouveau vide.");
            create_fichier(nom_fichier)
            return "avec difficulté"
        }
        else {
            panic!("Problem opening the default file: {error:?}");
        }
    });
    print!("Fichier {ouverture_terrain_result:?} ouvert");
    return "correctement"
}
 
fn create_fichier(nom_fichier:String)-> File {
    let mut fichier=File::create(&nom_fichier).expect("Problem creating the file:");
    let message_de_creation:String=format!("Voici le CAD du projet {}. Bonne chance! \n Ce document n'est pas conçu pour du DOS.",nom_fichier);
    fichier.write_all(message_de_creation.as_bytes()).expect("Impossible d'écrire du texte au nouveau projet.");
    return fichier;
}

fn create_json(nom_fichier:String)-> File {
    //Aller chercher la version la plus récente de github au main. https://github.com/felixthibault/4D-renderer/blob/main/Préférences.json
    let mut json=File::create_new(&nom_fichier).expect("Problem creating the file:");
    let data:String=request(https://github.com/felixthibault/4D-renderer/blob/main/Préférences.json);
    let v: Value = serde_json::from_str(data);
    json.write_all(v);
    //Ça va très sûrement peut-être bugger ici en format binaire
    return json;
}
/*Régler ce code pour l'update beta
fn actualise_json{
    //Se rappeler de merger les préférences actuelles avec les nouvelles
    let data:String= match version {
        "Scratch"=>"",//Vide, comme l'avenir de ce projet

        "alpha"=>
    }
}
*/
  
fn ecrire_texte_fichier(mut projet:File,texte:String)->std::io::Result<()>{
    projet.write(texte.as_bytes())?;
    //write!("Some bytes were written to {}",projet);
    Ok(())
}
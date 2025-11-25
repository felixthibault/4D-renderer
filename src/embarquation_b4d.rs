//! # Gestion du fichier binaire
//! 
//! Ce fichier gère la lecture et l'écriture des éléments contenus dans 
//! le fichier binaire .b4d, que ce soit des entités dimensionnelles ou 
//! des plans, croquis, assemblages.
//! Pour l'instant, le fichier JSON doit s'appeler Préférences, dans une version
//! ultérieure, il pourrait s'appeler autrement ou être rennomable.

#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{fs, io, io::ErrorKind};
use fs::File as File;
use serde::Deserialize;
use serde_json::{self, Value};
//use std::io::{BufReader,Bytes};


pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'embarquation_b4d' appelé, fonctionnel: Oui");
}

pub fn verifier_fichier(json:serde_json::Value)-> File{
    //! Vérification de l'existence du fichier b4d. 
    //! Retourner la référence du fichier pour pouvoir l'éditer

    //On regarde quel est le nom par défaut du fichier du CAD inscrit dans le json
    let binding = json.get("Default_project_name")
        .expect("Le fichier JSON devrait avoir l'index 'Default_project_name' ")
        .to_string();
    let default_project_name:&str= binding.as_str();
    //Vérifier si ce fichier existe, sinon on tombe par défaut à le créer
    let ouverture_terrain_result: File = File::open(default_project_name).unwrap_or_else(|error|
        match error.kind(){
            ErrorKind::NotFound => {
                print!("Fichier du projet par défaut non trouvé, écriture d'un nouveau vide.");
                create_b4d(default_project_name)
            },
        _ => panic!("Problem opening the default file: {error:?}"),
        });
    print!("Fichier {default_project_name} ouvert.");
    //On retourne le nom du fichier
    ouverture_terrain_result
        
}
fn create_b4d(file_name:&str)-> File {
    let mut fichier: File=File::create(&file_name).expect("Problem creating the file:");
    //Utiliser fs::write à la place de File::create et fs::write_all, va créer et écrire le
    // contenu du fichier. Si un fichier était déjà présent avec ce nom, il sera réécrit.
    let message_de_creation:String=format!("Voici le CAD du projet {}. Bonne chance! \n 
            Ce document n'est pas conçu pour du DOS.",file_name);
    fs::write(file_name, message_de_creation.as_bytes())
        .expect("Impossible d'écrire du texte au nouveau projet.");
    return fichier;
}

pub fn verifier_json()-> serde_json::Value{
    /*Il y a trois principales façons de déconstruire un fichier json avec Serde
        1- Ouvrir le fichier avec file=File::open("Préférences.json").  
            Lire ce fichier directement avec json:serde_json::Value=serde_json::from_reader(file).
            Et ensuite on peut prendre une clé de la structure avec json.get(key).
        2- Lire le fichier en string ou en bytes avec file=fs::read_to_string("Préférences.json").
            Transformer ce fichier en type Value avec json=fs::read_to_string(file)
            Et ensuite on peut lire une clé avec json.get(key).
        3- */

    let json_file:File= File::open("Préférences.json").unwrap_or_else(|error|{
        match error.kind(){
            ErrorKind::NotFound=>{
                //Si le fichier de préférences n'existe pas
                println!("Fichier des paramètres non trouvé, écriture d'un nouveau 
                    à partir de la version actuelle.");
                create_json("Préférences.json");
                File::open("Préférences.json").unwrap()
                },
            _=> panic!("Problem opening the json file: {error:?}"),//Vérifier si cette partie doit retourner un type File
        }
    });
    //Le fichier json existe ou vient d'être créé.
    print!("Fichier Préférences.json ouvert.");
    //Help: https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
    //Désérializer le fichier
    print!("Deserializing it...");
    //Optionnel: si le json est très gros, on peut ajouter un BufReader 
    // pour traiter un stream avec de multiples connections.
    let data:Configuration=serde_json::from_reader(json_file).expect("JSON was not well-formatted");
    print!("Done.");
    data
}
fn create_json(file_name:&str) {
    //Aller chercher la version la plus récente de github au main. https://github.com/felixthibault/4D-renderer/blob/main/Préférences.json
    //let data:&str=request("https:github.com/felixthibault/4D-renderer/blob/main/Préférences.json");
    let data: serde_json::Value=serde_json::json!({
                "Configuration": {
                        "Default_project_name": "terrain.b4d",
                        "Preferred_default_project_name": "Cube.b4d",

                        "Debugging": true,
                        "Visualisation": "Isométrique",
                        "Projection4DMergée": true,
                        "_option_unités": "Les valeurs possibles sont cm,mm,in,m",
                        "Unité": "cm"

                    },
                "Utilisateur": "Félix T",
                "Version": "alpha"
        });
    let json:File=File::create(file_name).expect("Problem creating the JSON file:");
    //Ça va très sûrement peut-être bugger ici en format binaire
    serde_json::to_writer_pretty(json, &data);
    //On ne peut renvoyer un type fichier puisque cela n'implémente pas Copy
}

enum json_data{Configuration}
//Désérializer le json en json_data
//#[allow(non_snake_case)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration{
    Default_project_name:String,
    Preferred_default_project_name:String,
    Debugging:bool,
    Testing:bool,
    Visualisation:String,
    Projection4DMergée:bool,
    _option_unités:String,
    Unité:String,
}

/*Régler ce code pour l'update beta
fn actualise_json{
    //Se rappeler de merger les préférences actuelles avec les nouvelles
    let data:String= match version {
        "Scratch"=>"",//Vide, comme l'avenir de ce projet

        "alpha"=>
    }
}*/
/*  
fn ecrire_texte_fichier(mut projet:File,texte:String)->std::io::Result<()>{
    projet.write(texte.as_bytes())?;
    //write!("Some bytes were written to {}",projet);
    Ok(())
}

pub unsafe trait FromBytes {
    fn write_to_type(f: &mut std::fs::File) -> Self;
}
unsafe impl FromBytes for u32 {
    fn write_to_type(f: &mut std::fs::File) -> u32 {
    let mut bytes = [0u8; 4];
        let bytes_read = f.read_exact(&mut bytes);
    unsafe { std::mem::transmute::<[u8; 4], u32>(bytes) }
}}

unsafe fn write_byte(mut projet:File,bytes:u8)->std::io::Result<()>{
    projet.write_all(bytes);
}

//Wrapper les valeurs trop grosse de données de plusieurs bytes dans un vecteur de byte

type Wrapper=Bytes;
fn wrapper<R>(byte:Wrapper<R>)->Vec<Wrapper>{
    vec![byte]
}
//Allouer de l'espace dans le fichier avant qu'un objet soit complètement prêt à être écrit. Simuler l'écriture des entités.
//https:docs.rs/serde_bytes_wrapper/latest/serde_bytes_wrapper/struct.Bytes.html
*/
pub fn panik() {
    panic!("crash and burn");
}
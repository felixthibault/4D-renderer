#[allow(unused_imports)]
#[allow(unused_variables)]
use std::{fs::File, io, io::ErrorKind};
use serde::Deserialize;

pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'embarquation_b4d' appelé, fonctionnel: Oui");
}

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier(fichier_type:&str)->&str {
    if fichier_type=="Demarrage"{
        //Vérification des fichiers de démarrage standarts.
        let parametre= File::open("Préférences.json").unwrap_or_else(|error|{
            match error.kind(){
                ErrorKind::NotFound=>{
                print!("Fichier des paramètres non trouvé, écriture d'un nouveau à partir de la version actuelle .");
                create_json("Préférences.json")
                },
                _=> panic!("Le fichier de préférences devrait être read-only."),
            }});
        let json: serde_json::Value = serde_json::from_reader(parametre).expect("Le fichier de préférences devrait être de format JSON");
        //Utiliser json comme structure pour accéder au fichier JSON.
        
        let preference_ouverture = json.get("Ouverture_sécuritaire").unwrap();
        let nom_par_defaut={
            if preference_ouverture==true{json.get("Default_project_name")
                    .expect("Le nom par défaut du projet devrait exister si l'ouverture sécuritaire est activée.")}
            else{json.get("Preferred_default_project_name")
                    .expect("Le nom du défaut devrait exister si l'ouverture sécuritaire est désactivée.")};
        }.unwrap();
        let nom_fichier=&nom_par_defaut;
    }
    else {
        //Ouverture du fichier spécifique
        let nom_fichier=fichier_type;
    }
    let ouverture_terrain_result = File::open(&nom_fichier).unwrap_or_else(|error|
        match error.kind(){
        ErrorKind::NotFound => {
            print!("Fichier du projet par défaut non trouvé, écriture d'un nouveau vide.");
            create_fichier(&nom_fichier);
            return 1//Avec problème
            },
        _ => panic!("Problem opening the default file: {error:?}"),
        });
    print!("Fichier {ouverture_terrain_result:?} ouvert");
    return 0//Sans problème
    }
 
fn create_fichier(nom_fichier:&str)-> File {
    let mut fichier=File::create(&nom_fichier).expect("Problem creating the file:");
    let message_de_creation:String=format!("Voici le CAD du projet {}. Bonne chance! \n Ce document n'est pas conçu pour du DOS.",nom_fichier);
    fichier.write_all(message_de_creation.as_bytes()).expect("Impossible d'écrire du texte au nouveau projet.");
    return fichier;
}

fn create_json(nom_fichier:&str)-> File {
    //Aller chercher la version la plus récente de github au main. https://github.com/felixthibault/4D-renderer/blob/main/Préférences.json
    let mut json=File::create_new(&nom_fichier).expect("Problem creating the file:");
    let data:String=request("https:github.com/felixthibault/4D-renderer/blob/main/Préférences.json");
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
}*/
  
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

unsafe fn EcrireUnByteFichier(mut projet:File,bytes:u8)->std::io::Result<()>{
    projet.write(bytes);
}

//Wrapper les valeurs trop grosse de données de plusieurs bytes dans un vecteur de byte

type Wrapper=Bytes;
fn wrapper<R>(byte:Wrapper<R>)->Vec<Wrapper>{
    vec![byte]
}
//Allouer de l'espace dans le fichier avant qu'un objet soit complètement prêt à être écrit. Simuler l'écriture des entités.
//https:docs.rs/serde_bytes_wrapper/latest/serde_bytes_wrapper/struct.Bytes.html
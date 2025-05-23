#[allow(unused_imports)]
#[allow(unused_variables)]
use std::{*, fs::File,io::*};

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier(){
    let ouverture_terrain_result = File::open("terrain.d4b").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            create_fichier("terrain.b4d":File).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}"); });
        }
        else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    print!("Fichier {ouverture_terrain_result:?} ouvert");
    
}
 
fn create_fichier(nom_fichier:File){
    let create_fichier=File::create(nom_fichier);
    return create_fichier;
}

fn écrire_texte_fichier(terrain,texte:String){
    terrain.write(b"texte");
}
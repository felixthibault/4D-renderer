use std::{*, fs::File,io::*};

//'! Implémentation des instances d'objets regroupés sous le nom d'une entité
pub fn verifier_fichier() {
    let mut ouverture_terrain_result = File::open("terrain.d4b");
    
    let ouverture_terrain = match ouverture_terrain_result {
        Ok(file) => print!("Ouverture du fichier {file}"),
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    print!("Aucun problème trouvé");
}

fn create_fichier(nom:String){
    terrain.write(b"some data");
}
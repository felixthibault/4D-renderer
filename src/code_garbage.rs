//! # Code garbage à conserver 
//! 
//! Ici sont regroupés tous les codes malfoutus qu'il est intéressant de 
//! placer et d'oublier. Bon débarras!

//Vérification des macros générants des objets
fn verif() {
    // --temporaire-- Exemple de création d'un objet avec des tags, rôles, permissions et position initiale
    let objet:&str = "Triangle";
    let permissions: HashMap<String, bool> = HashMap::from([("fixe".to_string(),true)]);
    let reference: String=String::new();
    let triangle: Polygone = Polygone::create_polygone(objet,reference, permissions);
    // Ajout de nouveaux éléments
}

//Fin mod d'entités mal foutues

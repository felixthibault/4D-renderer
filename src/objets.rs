///Structure des objets et entités utilisées dans le CAD
/// 
/// J'ai séparé la création d'une entité en plusieurs instances d'objets qui sont des
/// points, lignes, carrés, cubes, tesseracts. Chaque instance a sa propre structure 
/// et l'intégration de toute les instances donne l'entité.
/// 

//Début vrai code
use bevy::prelude::Component;
use std::fmt;
pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'objets' appelé, fonctionnel: Oui");
}

#[derive(Debug)]
#[derive(Component)]
pub struct Nom(String);
pub struct Fixe(bool);

#[derive(Debug)]
#[derive(Component)]
enum MesStructures{
    Entite, //Structure globale, rien n'est plus grand
    Point, //Structure la plus petite, constituant de base des autres
    Ligne, //Structure nécessitant deux Points pour être créée
    Polygone, //Structure ayant besoin de ligne ou de points pour se créer. 
            //L'algorithme des lignes dans une matrice et d'une matrice d'ordre 
            //ou de points détermine comment elle est générée 
    Polyedre //Structure 3D alternative du Polygone.
             //Elle nécessite une matrice d'ordre et de Polygones pour être générée.
}

#[derive(Debug)]
#[derive(Component)]
pub struct Position<T>{
    x:T,
    y:T,
    z:T,
}


//'! Structure générale des entités
pub struct Entite < T /* Type */>{
    
    pub nom:Nom,// Nom de l'entité
    tags:Option<Vec<String>>,// Tags associés à l'entité
    constituant:Vec<MesStructures>,//Objets contenus dans cette entité
    role:Vec<Nom>,// Type et rôles associés à l'entité
    //donnees:HashMap<String, String>,// Informations sous forme de clés booléennes (Ajouter des défauts)
}

//'! Structure des objets "Points"
pub struct Point<T>{
    pub nom:Nom,// Nom du point. Il n'y a pas de dimension inférieure dépendante du point
    x: T,  //Coordonnées associées au point
    y: T,
    z: T,
    etat:Fixe,
    //w: T,
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Lignes"
#[derive(Debug)]
pub struct Ligne{
    pub nom:Nom,// Nom de la ligne
    constituant:Vec<MesStructures>,//Intégration des points formant cette ligne
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polygones"
#[derive(Debug)]
pub struct Polygone{
    pub nom:Nom,// Nom de la figure en 2D
    constituant:Vec<MesStructures>,//Intégration des lignes formant ce polygone
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polyèdres"
#[derive(Debug)]
pub struct Polyedre{
    pub nom:Nom,// Nom de la figure en 3D
    constituant:Vec<MesStructures>,//Intégration des polygones formant ce polyèdre
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}


//'! Fonctions de création des objets
impl<T> Entite<T>{
    //Création d'une nouvelle entité si les références concordent
    fn new(nom:&str,pos:Position<T>)-> Entite<T> {
        Entite{nom:Nom(nom.to_string()),tags:None,constituant: vec![Point::new(nom,pos)], role: vec![Nom("point".to_string())]}
    }
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    // Modifier les tags
    fn changer_tags(&mut self, tag: Vec<String>) {self.tags=Some(tag);}
    fn ajouter_tags(&mut self, tag: &str) {
        if self.tags==None{self.changer_tags(vec![tag.to_string()]);}
        else{self.tags.as_mut().expect("on ne devrait pas ajouter de tags à un groupe vide").push(tag.to_string())
        //On extrait des tags vides ou pleines leur référence mutable et on pousse un  nouvel élément
        }
    }
    //Modifier les rôles
    fn changer_roles(&mut self, role:Vec<Nom>) {self.role=role;}
    fn ajouter_roles(&mut self, role:&str) {self.role.push(Nom(role.to_string()));}
    //Modifier les données
    //fn changer_donnees(&mut self, donnees:HashMap<String,String>) {self.donnees=donnees;}
    //fn ajouter_donnees(&mut self, clés:&str, donnée:&str) {self.donnees.insert(clés.to_string(),donnée.to_string());}
    // Afficher les détails de l'objet
    pub fn afficher(&self) {
        println!("Entité {{");
        println!("  Nom: {:?}", self.nom);
        println!("  Tags: {:?}", self.tags);
        println!("  Objets: {:?}", self.objets);
        println!("  Rôles: {:?}", self.role);
        println!("  Données: {:?}", self.donnees);
        println!("}}");
    }
    
    pub fn create_square(grosseur:i32)->Entite{
        let p1=Point::new("square", (-grosseur,-grosseur,0));
        let p2=Point::new("square", (grosseur,-grosseur,0));
        let p3=Point::new("square", (grosseur,grosseur,0));
        let p4=Point::new("square", (-grosseur,grosseur,0));
        Entite{nom:Nom("square".to_string()), constituant:vec![p1,p2,p3,p4]}
    }
}
impl<T> fmt::Display for Entite<T>{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Entite: {}", self.nom)
    }
}



impl<T> Point<T>{
    //Création d'un nouveau point selon les coordonnées
    fn new(nom:&str,pos:Position<T>)-> Point<T> {
        let (x,y,z)=(pos.x,pos.y,pos.z);
        Point{ nom:Nom(("point de"+nom.to_owned()).to_string()),
            x,y,z,etat:Fixe(false)}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    // Modifier x
    fn changer_x(&mut self, nouvelle_coordonnee: f32) {self.x=nouvelle_coordonnee;}
    // Modifier y
    fn changer_y(&mut self, nouvelle_coordonnee: f32) {self.y=nouvelle_coordonnee;}
    // Modifier z
    fn changer_z(&mut self, nouvelle_coordonnee: f32) {self.z=nouvelle_coordonnee;}
    // Modifier w
    fn changer_w(&mut self, nouvelle_coordonnee: f32) {self.w=nouvelle_coordonnee;}
    // Modifier les permissions
    fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}
    
    // Afficher les détails de l'objet
    pub fn afficher(&self) {
        println!("Point {{");
        println!("  Nom: {:?}", self.nom);
        println!("  x: {:?}", self.x);
        println!("  y: {:?}", self.y);
        println!("  z: {:?}", self.z);
        println!("  w: {:?}", self.w);
        println!("  Permissions: {:?}", self.permissions);
        println!("}}");
    }  
}

impl<T> fmt::Debug for Point<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Imprime la position et le nom du point
        //Permet de faire dbg!(point);
        f.debug_struct("Point")
                .field("nom", &self.nom)
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .finish()
    }
}

impl Ligne{
    //Création d'une nouvelle ligne selon les références (points, équations)
    fn create_ligne(
        nom:&'static str,
        reference:String,
        permissions:HashMap<String, bool>,
        )-> Self 
        {Ligne{nom,reference,permissions,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&'static str) {self.nom=nom;}
    //Modifier les références
    fn ajouter_reference(&mut self, nouvelle_référence:&str) {self.reference.push_str(nouvelle_référence);}
    fn changer_reference(&mut self, reference:String) {self.reference=reference;}
    //Modifier les permissions
    fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}

    // Afficher les détails de l'objet
    pub fn afficher(&self) {
        println!("Ligne {{");
        println!("  Nom: {:?}", self.nom);
        println!("  Référence: {:?}",self.reference);
        println!("  Permissions: {:?}", self.permissions);
        println!("}}");
    }  
}

impl Polygone{
    //Création d'un nouveau polygone selon les références
    fn create_polygone(
        nom:&'static str,
        reference:String,
        permissions:HashMap<String, bool>,
        )-> Self 
        {Polygone{nom,reference,permissions,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&'static str) {self.nom=nom;}
    //Modifier les références
    fn ajouter_reference(&mut self, nouvelle_référence:&str) {self.reference.push_str(nouvelle_référence);}
    fn changer_reference(&mut self, reference:String) {self.reference=reference;}
    //Modifier les permissions
    fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}

    // Afficher les détails de l'objet
    pub fn afficher(&self) {
        println!("Polygone {{");
        println!("  Nom: {:?}", self.nom);
        println!("  Référence: {:?}",self.reference);
        println!("  Permissions: {:?}", self.permissions);
        println!("}}");
    } 
}

impl Polyedre{
    //Création d'un nouveau polyèdre selon les références
    fn create_polyèdre(
        nom:&'static str,
        reference:String,
        //permissions:HashMap<String, bool>,
        )-> Self 
        {Polyedre{nom,reference,permissions,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&'static str) {self.nom=nom;}
    //Modifier les références
    fn ajouter_reference(&mut self, nouvelle_référence:&str) {self.reference.push_str(nouvelle_référence);}
    fn changer_reference(&mut self, reference:String) {self.reference=reference;}
    //Modifier les permissions
    fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}

    // Afficher les détails de l'objet
    pub fn afficher(&self) {
        println!("Polyèdre {{");
        println!("  Nom: {:?}", self.nom);
        println!("  Référence: {:?}",self.reference);
        println!("  Permissions: {:?}", self.permissions);
        println!("}}");
    } 
}
//! # Structure des objets et entités utilisées dans le CAD
//! 
//! J'ai séparé la création d'une entité en plusieurs instances d'objets qui sont des
//! points, lignes, polygones, polyèdres. Il faudra attendre une prochaine version pour
//! voir apparaître les polychores (4D).
//! Chaque instance a sa propre structure et l'intégration de toute les instances donne 
//! ce qu'on appelle une entité. À ne pas mélanger avec la structure Entite qui, elle 
//! est un croquis ou une forme extrudée.

//Début vrai code
use bevy::{math::primitives::Polygon, prelude::Component};
use std::{fmt,ops::Neg};
use num_traits::Zero;


pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println!("Module 'objets' appelé, fonctionnel: Oui");
}

#[derive(Debug)]
#[derive(Component)]

//Structures d'un nom et de l'état fixe ou non d'une structure
#[derive(Clone)]
pub struct Nom(String);
#[derive(Debug,Clone)]
pub struct Fixe(bool);

#[derive(Debug)]
#[derive(Component)]
pub enum MesStructures{
    Entite, //Structure globale, rien n'est plus grand
    Point, //Structure la plus petite, constituant de base des autres
    Ligne, //Structure nécessitant deux Points pour être créée
    Polygone, //Structure ayant besoin de ligne ou de points pour se créer. 
            //L'algorithme des lignes dans une matrice et d'une matrice d'ordre 
            //ou de points détermine comment elle est générée 
    Polyedre //Structure 3D alternative du Polygone.
             //Elle nécessite une matrice d'ordre et de Polygones pour être générée.
}

#[derive(Debug,Component)]
pub struct Position<T>{
    //Tuple d'une position 3D d'un point. Cela permet, comme dans desmos, de faire (p1.x,p1.y,p1.z)
    x:T,
    y:T,
    z:T,
}


//'! Structure générale des entités
pub struct Entite {
    //Une entité peut être un sketch ou croquis regroupant plusieurs objets, tels que des points, lignes ou polygones
    //ou bien l'extrusion 3D de ce croquis.
    pub nom:Nom,// Nom de l'entité
    tags:Option<Vec<String>>,// Tags associés à l'entité
    constituant:Vec<MesStructures>,//Objets contenus dans cette entité
    role:Vec<Nom>,// Type et rôles associés à l'entité
    //donnees:HashMap<String, String>,// Informations sous forme de clés booléennes (Ajouter des défauts)
}

//'! Structure des objets "Points"
#[derive(Clone)]
pub(crate) struct Point<H> {
    pub nom:Nom,// Nom du point. Il n'y a pas de dimension inférieure dépendante du point
    x: H,  //Coordonnées associées au point
    y: H,
    z: H,
    etat:Fixe,
    //w: H,
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Lignes"
pub struct Ligne<H>{
    pub nom:Nom,// Nom de la ligne
    p1:Point<H>,//Intégration des points formant cette ligne
    p2:Point<H>,
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polygones"
pub struct Polygone<H>{
    pub nom:Nom,// Nom de la figure en 2D
    constituant:Vec<Ligne<H>>,//Intégration des lignes formant ce polygone
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polyèdres"
pub struct Polyedre<H>{
    pub nom:Nom,// Nom de la figure en 3D
    constituant:Vec<Polygone<H>>,//Intégration des polygones formant ce polyèdre
    //permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

pub struct Plan<T>(T);
pub struct Vecteur<T>(T,T,T);
pub struct Croquis<T>{
    origine: Position<T>,
    normale: Position<T>,
    equation:  Plan<T>, //Équation d'un plan Ax+By+Cz+D=0
    //Un croquis est un plan où les instances d'objets peuvent être placés pour une construction
    //Par la définition 3D, il est défini par un point d'origine, son vecteur directeur et 
    //une constante t. Il peut aussi être une équation mêlant tout ça.
}


//'! Fonctions de création des objets

impl<T:Zero> Position<T>{
    pub fn new(x:T,y:T,z:T)->Self{
        Position {x, y, z}
    }
    fn null()->Self{
        Position{x:T::zero(),y:T::zero(),z:T::zero()}
    }

}

impl Entite{
    //Création d'une nouvelle entité si les références concordent
    fn new(nom:&str)-> Self {
        Entite{nom:Nom(nom.to_string()),
                tags:None,
                constituant: vec![MesStructures::Point],
                role: vec![Nom("point".to_string())]
            }
    }
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    // Modifier les tags
    fn changer_tags(&mut self, tag: Vec<String>) {self.tags=Some(tag);}
    fn ajouter_tags(&mut self, tag: &str) {
        if self.tags==None{self.changer_tags(vec![tag.to_string()]);}
        else{self.tags.as_mut().expect("on ne devrait pas ajouter de tags à un groupe vide").push(tag.to_string())
        //On extrait des tags vides ou pleines leur référence mutable et on pousse un  nouvel élément.
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
        println!("  Constituants: {:?}", self.constituant);
        println!("  Rôles: {:?}", self.role);
        //println!("  Données: {:?}", self.donnees);
        println!("}}");
    }
}

impl<T> Point<T>{
    //Création d'un nouveau point mobile selon les coordonnées
    pub fn new(nom:&str,pos:Position<T>)-> Self {
        let (x,y,z)=(pos.x,pos.y,pos.z);
        Point{ nom:Nom(("point de ".to_owned()+nom).to_string()),
            x,y,z,etat:Fixe(false)}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    // Modifier x
    fn changer_x(&mut self, nouvelle_coordonnee: T) {self.x=nouvelle_coordonnee;}
    // Modifier y
    fn changer_y(&mut self, nouvelle_coordonnee: T) {self.y=nouvelle_coordonnee;}
    // Modifier z
    fn changer_z(&mut self, nouvelle_coordonnee: T) {self.z=nouvelle_coordonnee;}
    // Modifier w
    //fn changer_w(&mut self, nouvelle_coordonnee: T) {self.w=nouvelle_coordonnee;}
    
    //Modifier l'état fixe
    fn fixer(&mut self, fixe:bool){self.etat=Fixe(fixe);}
}

impl<H> Ligne<H>{
    //Création d'une nouvelle ligne selon les références (points, équations)
    fn new(nom:&str, p1:Point<H>, p2:Point<H>)-> Self {Ligne{nom:Nom(nom.to_string()),p1,p2}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    //Modifier les références de points
    fn changer_constituant(&mut self, nouvelle_reference:Vec<Point<H>>) where Point<H>: Clone{
        self.p1=nouvelle_reference[0].clone();
        self.p2=nouvelle_reference[1].clone();
    }
}

impl<H:Zero> Polygone<H>{
    //Création d'un nouveau polygone selon les références
    fn new(nom:&str,constituant:Vec<Ligne<H>>,)-> Self {Polygone{nom:Nom(nom.to_string()),constituant}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    //Modifier les références de lignes
    fn ajouter_constituant(&mut self, nouvelle_référence:Ligne<H>) {self.constituant.push(nouvelle_référence);}
    fn changer_constituant(&mut self, reference:Vec<Ligne<H>>) {self.constituant=reference;}
    //Modifier les permissions
    //fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    //fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}

    pub fn create_square(grosseur:H)-> Self
        where H:Neg+Copy+Neg<Output = H>, <H as Neg>::Output: Zero, Point<H>:Clone,{
        let p1=Point::new("square", Position::new(-grosseur,-grosseur,H::zero()));
        let p2=Point::new("square", Position::new(grosseur,-grosseur,H::zero()));
        let p3=Point::new("square", Position::new(grosseur,grosseur,H::zero()));
        let p4=Point::new("square", Position::new(-grosseur,grosseur,H::zero()));
        let l1=Ligne::new("p1-p2",p1.clone(),p2.clone());
        let l2=Ligne::new("p2-p3",p2.clone(),p3.clone());
        let l3=Ligne::new("p3-p4",p3.clone(),p4.clone());
        let l4=Ligne::new("p4-p1",p4.clone(),p1.clone());
        Polygone::new("square", vec![l1,l2,l3,l4])
    }
}

impl<H> Polyedre<H>{
    //Création d'un nouveau polyèdre selon les références
    fn new(nom: &str,constituant:Vec<Polygone<H>>,)-> Self {Polyedre{nom:Nom(nom.to_string()),constituant,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&str) {self.nom=Nom(nom.to_string());}
    //Modifier les références de polygones
    fn ajouter_constituant(&mut self, nouvelle_référence:Polygone<H>) {self.constituant.push(nouvelle_référence);}
    fn changer_constituant(&mut self, reference:Vec<Polygone<H>>) {self.constituant=reference;}
    //Modifier les permissions
    //fn ajouter_permission(&mut self, clés: &str, booléen: bool) {self.permissions.insert(clés.to_string(), booléen);}
    //fn changer_permission(&mut self, permission: HashMap<String, bool>) {self.permissions=permission;}
}

//Implémentation de débogage des structures

impl fmt::Display for Nom{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, "{}",self)
    }
}


impl fmt::Display for Entite{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Entite: {:?}", self.nom)
    }
}

impl<T:fmt::Debug> fmt::Debug for Point<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Imprime la position et le nom du point
        //Permet de faire dbg!(point);
        f.debug_struct("Point=")
                .field("nom", &self.nom)
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                //.field("w", &self.w)
                .field("fixe", &self.etat)
                .finish()
    }
}

impl<H:fmt::Debug> fmt::Debug for Ligne<H>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Imprime les constituants et le nom de la ligne
        //Permet de faire dbg!(ligne);
        f.debug_struct("Ligne=")
                .field("nom", &self.nom)
                .field("point 1", &self.p1)
                .field("point 2", &self.p2)
                .finish()
    }
}

impl<H:fmt::Debug> fmt::Debug for Polygone<H>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Imprime les constituants et le nom du polygone
        //Permet de faire dbg!(polygone);
        f.debug_struct("Polygone=")
                .field("nom", &self.nom)
                .field("constituants", &self.constituant)
                .finish()
    }
}

impl<H:fmt::Debug> fmt::Debug for Polyedre<H>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Imprime les constituants et le nom du polyèdre
        //Permet de faire dbg!(polyedre);
        f.debug_struct("Polyèdre=")
                .field("nom", &self.nom)
                .field("constituants", &self.constituant)
                .finish()
    }
}
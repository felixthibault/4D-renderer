//! # Fonctions des transformations matriciels des objets.
//! 
//! J'ai séparé la création d'une entité en plusieurs instances d'objets qui sont des
//! points, lignes, carrés, cubes, tesseracts. Chaque instance a sa propre structure 
//! et l'intégration de toute les instances donne l'entité.

#[cfg(not(target_arch = "wasm32"))]
//use std::*;
use std::{collections::HashMap, fs::*, io::prelude::*, os::unix::fs::FileExt, string::*};
use std::ops::{Add,Mul};
use bevy::{prelude::*,render::{render_asset::RenderAssetUsages,render_resource::{
    Extent3d, TextureDimension, TextureFormat},},};
use Option::Some;
use num_traits::{ToPrimitive,zero};
    
#[path = "main.rs"]
mod embarquation_b4d;
use embarquation_b4d::*;



//'! Structure générale des entités
#[derive(Debug)]
pub struct Entité{
    pub nom:&'static str,// Nom de l'entité
    tags:Vec<String>,// Tags associés à l'entité
    objets:String,//Objets contenus dans cette entité
    role:Vec<String>,// Type et rôles associés à l'entité
    donnees:HashMap<String, String>,// Informations sous forme de clés booléennes (Ajouter des défauts)
}

//'! Structure des objets "Points"
#[derive(Debug)]
pub struct Point{
    pub nom:&'static str,// Nom du point
    //Pas de tags car il n'y a pas de dimension inférieur dépendante du point
    x: f32,  //Coordonnées associées au point
    y: f32,
    z: f32,
    w: f32,
    permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Lignes"
#[derive(Debug)]
pub struct Ligne{
    pub nom:&'static str,// Nom de la ligne
    reference:String,//Intégration des points formant cette ligne
    permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polygones"
#[derive(Debug)]
pub struct Polygone{
    pub nom:&'static str,// Nom de la figure en 2D
    reference:String,//Intégration des lignes formant ce polygone
    permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polyèdres"
#[derive(Debug)]
pub struct Polyèdre{
    pub nom:&'static str,// Nom de la figure en 3D
    reference:String,//Intégration des polygones formant ce polyèdre
    permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}

//'! Structure des objets "Polychores"
#[derive(Debug)]
pub struct Polychore{
    pub nom:&'static str,// Nom de la figure en 4D
    reference:String,//Intégration des lignes formant ce polychore
    permissions:HashMap<String, bool>,// Permissions sous forme de clés booléennes
}




//'! Fonctions de création des objets

impl Entité{
    //Création d'une nouvelle entité si les références concordent
    fn create_entity(
        nom:&'static str,
        tags:Vec<String>,
        objets:String,
        role:Vec<String>,
        donnees:HashMap<String, String>,
        )-> Self 
        {Entité{nom,tags,objets,role,donnees,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&'static str) {self.nom=nom;}
    // Modifier les tags
    fn changer_tags(&mut self, tag: Vec<String>) {self.tags=tag;}
    //Les instances d'objets présents ne peuvent être modifiés une fois créés
    //Modifier les rôles
    fn changer_roles(&mut self, role:Vec<String>) {self.role=role;}
    fn ajouter_roles(&mut self, role:String) {self.role.push(role);}
    //Modifier les données
    fn changer_donnees(&mut self, donnees:HashMap<String,String>) {self.donnees=donnees;}
    fn ajouter_donnees(&mut self, clés:&str, donnée:&str) {self.donnees.insert(clés.to_string(),donnée.to_string());}
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
}

impl Point{
    //Création d'un nouveau point selon les coordonnées
    fn create_point(
        nom:&'static str,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        permissions:HashMap<String, bool>,
        )-> Self 
        {Point{nom,x,y,z,w,permissions,}}
    //Modifier le nom
    fn changer_nom(&mut self, nom:&'static str) {self.nom=nom;}
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

impl Polyèdre{
    //Création d'un nouveau polyèdre selon les références
    fn create_polyèdre(
        nom:&'static str,
        reference:String,
        permissions:HashMap<String, bool>,
        )-> Self 
        {Polyèdre{nom,reference,permissions,}}
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

impl Polychore{
    //Création d'un nouveau polychore selon les références
    fn create_polychore(
        nom:&'static str,
        reference:String,
        permissions:HashMap<String, bool>,
        )-> Self 
        {Polychore{nom,reference,permissions,}}
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
        println!("Polychore {{");
        println!("  Nom: {:?}", self.nom);
        println!("  Référence: {:?}",self.reference);
        println!("  Permissions: {:?}", self.permissions);
        println!("}}");
    } 
}


//Vérification des macros générants des objets
fn verif() {
    // --temporaire-- Exemple de création d'un objet avec des tags, rôles, permissions et position initiale
    let objet:&str = "Triangle";
    let permissions: HashMap<String, bool> = HashMap::from([("fixe".to_string(),true)]);
    let reference: String=String::new();
    let triangle: Polygone = Polygone::create_polygone(objet,reference, permissions);
    // Ajout de nouveaux éléments

    // Affichage des détails de l'objet
    triangle.afficher();
}


//Fonctions pour aider la production
pub fn println(msg:&str){
    println!("{}",msg);
}
pub fn print(msg:&str){
    print!("{}",msg);
}
pub fn ReportError(message:&str,code:String){
    //Afficher fenêtre contenant une erreur mineure
    //Pour l'instant:
    println!("{} {}.",message,code);
}

pub fn panik() {
    println("crash ans burn");
    panic!("crash and burn");
}

pub fn unreachable(x: Void) -> ! {
    match x {}
}

pub unsafe fn do_nothing(_:Void) -> !{
    return Void
}
pub mod Convert{
    pub fn convert_f64_to_u64(x: f64) -> Option<u64> {
        let y = x as u64;
        if y as f64 == x {
            Some(y)
        } else {
            None
        }
    }
    pub fn convert_f32_to_u32<T:Into<f32> + Copy>(x: T) -> Option<u32> {
        let y = x.into() as f32 as u32;
        if y as f32 == x.into() as f32 {
            Some(y)
        } else {
            None
        }
    }
    pub fn convert_to_u16<T: Into<f64>>(x:T)->u16{
        x.into() as u16
    }
    pub fn convert_to_usize<T: Into<f64>>(x:T)->usize{
        x.into() as usize
    }
    pub fn convert_to_isize<T: Into<f64>>(x:T)->isize{
        x.into() as isize
    }
}
fn dereferencer(reference:String)->Option<Vec<String>>{
    //Méthode pour déférencer les entités depuis les références du fichier binaire séparés par des virgules.
    //Crée des structures temporaires de toutes les références
    //Retourner une liste avec l'instance nommée des sous-structures (comment?: le faire en boucle, vérifier que les structures ne sont pas effacées lors de la boucle)
    /*Autre idée pour simplifier les coordonnées: représenter les entités déférencées temporairement avec 4 matrices, soit une matrice de sommets (4 lignes de hauteur par x points de long),
      une matrice d'arêtes (chaque ligne représente une arrête et comporte 4 éléments: 2 premières colonnes= référence ou numéro des sommets, les x autres= les références ou numéro des faces
      contenant cette arête), une matrice de faces (chaque ligne représente une face et comporte naturellement 2 colonnes= référence ou numéro des polyèdres contenant cette face), finalement
      une matrice de polyèdres (chaque ligne représente un polyèdre et comporte x colonnes= référence ou numéro des polychores contenant ce polyèdre). Le reste des données modifiées pourrait
      être généré d'une autre façon (laquelle?) ou avec des références comme plus haut.
    */
    return SousStructures
}

fn ScalableFloatMatrix(scalaire:f32,mut matrice:&[&[f32]]){
    //Multiplication d'une matrice par un scalaire float. Panique si les types ne sont pas tous des float.
    for mut j in &mut matrice{
        for i in 0..j.len(){
            j[i]*=scalaire;
        }
    }
}

fn ScalableIntMatrix(scalaire:u32,mut matrice:&[&[usize]]){
    //Multiplication d'une matrice d'entiers par un scalaire int non signé. Panique si les types ne sont pas tous des usize.
    for mut j in &mut matrice{
        for i in 0..j.len(){
            j[i]*=scalaire;
        }
    }
}

pub fn scalable_matrix<T: Copy + AddAssign + Mul<Output=T>>(scalaire:T, &mut matrice:&[&[T]]){
    //Multiplication d'une matrice de nombre par un scalaire de même nature. Méthode ne retournant rien. Panique si les types ne sont pas tous les mêmes.
    for j in &matrice{
        for mut i in &j{
            i*=scalaire;
        }
    }
}

pub(super) fn MultiplicationFloatMatrices(matrice1:&[Vec<f32>],matrice2:&[Vec<f32>])->Vec<Vec<f32>>{
    //Multiplie des matrices f32 de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    if matrice1.is_empty() || matrice2.is_empty() || matrice1[0].len()!=matrice2.len(){
        print("Multiplication de matrices incompatibles");
        println!("Longueur de matrice 1:{}, hauteur de matrice 2:{}",matrice1[0].len(),matrice2.len());
        panik();
    }
    let mut matrice3:Vec<Vec<f32>>=Vec::new();
    let mut calcul:f32;
    for j in 0..matrice1.len(){
        matrice3.push(Vec::new());
        for i in 0..matrice2[0].len(){
            calcul=0.0;
            for case in 0..matrice1[j].len(){
                calcul+=a[j][case]*matrice2[case][i];
            matrice3[j].push(calcul);
            }    
        }
    }
    return matrice3
}

pub(super) fn MultiplicationIntMatrices(matrice1:&[Vec<usize>],matrice2:&[Vec<usize>])->Vec<Vec<usize>>{
    //Multiplie des matrices d'uX de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    if matrice1.is_empty() || matrice2.is_empty() || matrice1[0].len()!=matrice2.len(){
        print("Multiplication de matrices incompatibles");
        println!("Longueur de matrice 1:{}, hauteur de matrice 2:{}",matrice1[0].len(),matrice2.len());
        panik();
    }
    let mut matrice3:Vec<Vec<usize>>=Vec::new();
    let mut calcul:usize;
    for j in 0..matrice1.len(){
        matrice3.push(Vec::new());
        for i in 0..matrice2[0].len(){
            calcul=0;
            for case in 0..matrice1[j].len(){
                calcul+=matrice1[j][case]*matrice2[case][i];
            matrice3[j].push(calcul);
            }    
        }
    }
    return matrice3
}

pub fn multiplication_matrices<T: Copy + AddAssign + Mul<Output=T> + num::Zero>(a:&[&[T]],b:&[&[T]])->Result<Vec<Vec<T>>>{
    //Multiplie des matrices d'unités inconnues de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    //https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-operations-sur-les-matrices-m1467#multiplication
    if a.is_empty() || b.is_empty() || a[0].len()!=b.len(){
        print("Multiplication de matrices incompatibles");
        println!("Longueur de matrice 1:{}, hauteur de matrice 2:{}",a[0].len(),b.len());
        panik();
        return Err("Incompatible matrix dimensions")
    }
    let mut matrice_resultat:Vec<Vec<T>>=vec![vec![T::zero();b[0].len()];a.len()];
    for j in 0..a.len(){
        for i in 0..b[0].len(){
            for case in 0..a[j].len(){
                matrice_resultat[j][i]+=a[j][case]*b[case][i];
            }    
        }
    }
    return Ok(matrice_resultat)
}
pub fn addition_vecteurs<T>(a:&[T], b:&[T])->Result<Vec<T>> where T: Add<Output=T>,{
    //Retourne l'addition de deux slices dans un vecteur de même longueur. Retourne une erreur si les longueurs ne correspondent pas. Peut prendre des vecteurs vides.
    if  a.len()!=b.len(){
        if a.is_empty(){ Ok(b) } else if b.is_empty() { Ok(a) }
        print("Addition de vecteurs incompatibles");
        println!("Longueur de vecteur 1:{}, longueur de vecteur 2:{}",a.len(),b.len());
        return Err("Incompatible vector dimensions")
    } else if a.is_empty(){ Ok(Vec::new()) } //Les deux sont vides
    let mut resultat:Vec<T>=Vec::new();
    for i in 0..a.len(){
        resultat.push(a[i]+b[i]);
    }
    return Ok(resultat)
}
pub fn addition_matrices<T>(a:&[&[T]], b:&[&[T]])->Result<Vec<Vec<T>>> where T: Add<Output=T>,{
    //Retourne l'addition de deux matrices dans une matrice de même longueur. Retourne une erreur si les longueurs ne correspondent pas.
    if  a.len()!=b.len() || a[0].len()!=b[0].len(){
        if a.is_empty(){ Ok(b) } else if b.is_empty() { Ok(a) }
        print("Addition de matrice incompatibles");
        println!("Longueur de vecteur 1:{}, longueur de vecteur 2:{}",a.len(),b.len());
        println!("Hauteur de vecteur 1:{}, hauteur de vecteur 2:{}",a[0].len(),b[0].len());
        return Err("Incompatible matrix dimensions")
    } else if b.is_empty(){ return Ok(Vec::new()) //Les deux sont vides}
    let mut resultat:Vec<Vec<T>>=vec![vec![0;a[0].len()];a.len()];
    for i in 0..a.len(){
        for j in 0..a[0].len(){
            resultat[i][j]+=a[i][j]+b[i][j];
        }
    }
    return Ok(resultat)
}

pub fn completer_matrice_carre<T: num::Zero>(&mut matrice:&[[T]]){
    //Méthode modifiant une matrice non-carrée envoyée en paramètre. Essentiellement transforme de matrice rectangulaire à matrice carrée en ajoutant des lignes de zéros#.
    for i in 0..matrice[0].len()-matrice.len(){
        matrice.push(vec![T::zero();i]);
    }
}

mod Trigo{
    pub(super)fn sin<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //Retourne le sin de theta radians. Panique si le type d'angle n'est pas f32. Prend des floats ou integers.
        print!("Receive sin function. Angle of {} degrees. ", theta);
        (theta.into() as f32).sin()
    } 
    pub(super) fn cos<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //retourne le cos de theta radians
        print!("Receive cos function. Angle of {} degrees. ", theta);
        (theta.into() as f32).cos()
    }

    pub fn sin_f64<S: ToPrimitive + std::fmt::Display>(theta:S)->f64{
        //Retourne le sin en f64 de n'importe quel nombre entré.
        print!("Receive sin function. Angle of {} degrees. ", theta);
        theta.to_f64().expect("Échec de la conversion en f64").sin()
    }
    pub fn cos_f64<S: ToPrimitive + std::fmt::Display>(theta:S)->f64{
        //Retourne le cos en f64 de n'importe quel nombre entré.
        print!("Receive cos function. Angle of {} degrees. ", theta);
        theta.to_f64().expect("Échec de la conversion en f64").cos()
    }

    pub(super) fn tan<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //Retourne la pente à theta radians
        print!("Receive tan function. Angle of {} degrees. ", theta);
        (theta.into() as f32).tan()
    }
    pub(super) fn sec<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //Retourne la sécante à theta radians
        print!("Receive sec function. Angle of {} degrees. ", theta);
        (theta.into() as f32).sec()
    }
    pub(super) fn cosec<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //Retourne la cosécante à theta radians
        print!("Receive cosec function. Angle of {} degrees. ", theta);
        (theta.into() as f32).csc()
    }
    pub(super) fn cot<S: Into<f64> + std::fmt::Display>(theta:S)->f32{
        //Retourne la pente à theta radians
        print!("Receive cot function. Angle of {} degrees. ", theta);
        (theta.into() as f32).cot()
    }
    pub(super) fn arctan<S: Into<f64> + std::fmt::Display>(x:S)->f32{
        //Retourne l'angle theta d'une pente x
        print!("Receive arctan function. Pente of {}. ", x);
        (x.into() as f32).atan()
    }
    pub(super) const atan: fn(f32) -> f32 = arctan;//Pointer d'un f32 vers arctan
    
    pub(super) fn arcsin<S: Into<f64> + std::fmt::Display>(x:S)->f32{
        //Retourne l'angle theta d'une sine inverse x
        print!("Receive arcsin function. Coordinate of {}. ", x);
        (x.into() as f32).asin()
    }
    pub(super) const asin: fn(f32) -> f32 = arcsin;
    pub(super) fn arccos<S: Into<f64> + std::fmt::Display>(x:S)->f32{
        //Retourne l'angle theta d'une cos inverse x
        print!("Receive arccos function. Coordinate of {}. ", x);
        (x.into() as f32).acos()
    }
    pub(super) const acos: fn(f32) -> f32 = arccos;
}

//'! Transformation des points
/* Un matrice de dimension n est une suite de x vecteurs que l'on peut appliquer une transformation.
Un point peut aussi être une matrice de coordonnées (4 lignes et une colonne).
Les matrices peuvent se multiplier entre elles pour former une nouvelle matrice de dimension différente issue d'une transformation. Accumuler ces transformations pour les modification de croquis.
Pour une rotation ou d'autres transformations similaires, transformer d'abord le point en matrice est pratique.
https://web.archive.org/web/20091027131421/http://geocities.com/evilsnack/matrix.htm
https://en.wikipedia.org/wiki/Transformation_matrix
https://en.wikipedia.org/wiki/Plane_of_rotation#Double_rotations
https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-matrices-de-transformation-m1432
https://bibnum.publimath.fr/IST/IST83028.pdf
https://chatgpt.com
*/
    


pub(super) mod Transformation<S>{
    //Pour l'instant essayer d'avoir en argument des matrices ou scalaires d'int ou float. Par la suite voir si les entités sont nécessaires en arguments.
    //Le type de mesure envoyé pour la transformation devrait être le même que l'unité de la matrice.
    pub mod Matrice{
        pub fn translation_simple<T: Add<Output=T>>(mesure:&[T], &mut point:&[T]){
            //Méthode effectuant une translation simple sur un seul point. Format des vecteurs: x,y,z,w
            let mut facteur=mesure;
            if mesure.len()<4{
                //Manque de dimensions, ajouter des 0
                for dimension in 0..4-mesure.len(){facteur.push(0);}
            } else if mesure.len()>4{
                //Ce CAD est limité à 4 dimension calmdown!
                while facteur.len()!=4{facteur.pop();}
            }
            if point.len()<4{
                //Manque de dimensions, ajouter des 0
                for dimension in 0..4-point.len(){point.push(0);}
            }
            let point=addition_vecteurs(point,facteur);
        }
        
        pub fn translation<T: Into<f64> + Add<Output=T> + Mul<Output=T> + Clone + num::Zero>(mesure:&[S], &mut Entite:&[&[T]]){
            //Méthode effectuant une translation sur une matrice de points vectoriels (donc des points étalés dans un vecteur à la verticale et les points sont alignés en longueurs).
            //La matrice de translation est 5x5 donc Entite.len()==4

            //Changer T en f64 puis revenir en T
            let mut facteur=mesure.clone();
            if mesure.len()<4{
                //Manque de dimensions, ajouter des 0
                for dimension in 0..4-mesure.len(){facteur.push(T::zero());}
            } else if mesure.len()>4{
                //Ce CAD est limité à 4 dimension calmdown!
                while facteur.len()!=4{facteur.pop();}
            }
            let facteur:Vec<_>=vec![ vec![1,0,0,0,facteur[0]],
                                     vec![0,1,0,0,facteur[1]],
                                     vec![0,0,1,0,facteur[2]],
                                     vec![0,0,0,1,facteur[3]],
                                     vec![0,0,0,0,1] ];
            
        }
        
        pub fn stretching<T>(const mesure:&[T], const Entite:&[&[T]])->Vec<Vec<T>>{
            //Le facteur de mise à l'échelle doit lui-même être une matrice de longueur entre 1 et 4.

            // À RÉANALYSER
            let mut facteur:Vec<Vec<T>>=Vec::new();
            for dimension in 0..mesure.len(){
                facteur.push(vec![0;4]);
                facteur[dimension][dimension]=mesure[dimension];
            }
            if mesure.len()<4{
                //Ajouter des zéros pour compléter en carré
                let facteur=;
            }
            else if mesure.len()==4{
                //On fait f*ckall c'est déjà bon
                do_nothing()
            }
            else if mesure.len()>=4{
                //Ce CAD est limité à 4 dimension calmdown!
                ReportError("Trop de dimensions ajoutées dans cette mise en échelle", format!("{:?}",mesure));
            else {
                //Math is not mathing, please what is going on
                panik();
            }
            return MultiplicationTMatrices(facteur:&[&[T]],Entite:&[&[T]])
        }
        pub fn RotationDouble<T>(theta:T, phi:T,Entite:&[&[T]],plan1:&[&[T]],plan1:&[&[T]],origine:Vec<T>)->Vec<Vec<T>>{
            //https://fr.wikipedia.org/wiki/Rotation_en_quatre_dimensions
            //https://en.wikipedia.org/wiki/Plane_of_rotation#Double_rotations
            /*For example a rotation of α in the xy-plane and β in the zw-plane is given by the matrix [[cos(α),-sin(α),0,0],[sin(α),cos(α),0,0],[0,0,cos(β),-sin(β)],[0,0,sin(β),cos(β)]] */
            
        }
        pub fn rotation_arbitraire<T>(thetas:&[T], &mut Entite:&[&[T]], origine:Vec<T>){
            //Méthode effectuant une rotation multi-plans sur une entité constituée de points à la verticale, donc de hauteur 4. Theta devrait être de longueur 6, sinon sera ajusté, un angle par plan de rotation:6.
            //Puisque l'on peut considérer qu'une rotation autour d'un plan arbitraire (vecteur non aligné sur une dimension spécifique) est une suite de rotation du nombre de plan possible, cette fonction effectue autant de rotation que spécifié en sautant les zéros.
            //L'ordre des plans est comme suit: "xy","xz","xw","yz","yw","zw". Ainsi, ce sont des rotations simples (doubles isocliniques) qui sont effectuées ici. Renvoyer à rotation_simple.
            //https://articulatedrobotics.xyz/tutorials/coordinate-transforms/rotations-3d/
            for theta in thetas{
                if convert_to_u16(theta)!=0{
                    
                }
            }
        }
        pub fn rotation_un_axes<T>(angle:T,Entite:Vec<Vec<T>>,plan:&str,origine:Vec<T>)->Vec<Vec<T>>{
            //Panique si l'origine de rotation n'est pas de longueur 4 ou que l'angle n'a pas la même unité que l'entité.
            //Retourne une matrice de même dimension que l'originale. L'angle doit être en radian and l'axe doit être un plan deux dimensions
            //https://quaternions.online/
            //https://math.stackexchange.com/questions/1402362/can-rotations-in-4d-be-given-an-explicit-matrix-form
            /*Une rotation dans un plan signifie que les composantes associées à ces dimensions dans une entité sont modifiés alors que les autres axes dimensions restent fixes.
            Pour une rotation dans le plan xy, les composantes x et y des points seront modifiées alors que les z et w ne le seront pas. La différence avec le 3D est que deux axes (donc un plan) restent inchangés à la place d'un seul. */
            if origine.len()!=4{ReportError("Nombre de dimensions incorrect à l'origine de rotation",format!("{:?}",origine)}
            let mut facteur:Vec<Vec<T>>=Vec::new();
            let sin:Trigo::sin(angle:T);
            let cos:Trigo::cos(angle:T);
            match plan{
                "xy"|"yx"=>facteur.push(vec![cos,-sin,0,0],
                                        vec![sin,cos,0,0],
                                        vec![0,0,1,0],
                                        vec![0,0,0,1]),
                
                "xz"|"zx"=>facteur.push(vec![cos,0,-sin,0],
                                        vec![0,1,0,0],
                                        vec![sin,0,cos,0],
                                        vec![0,0,0,1]),
                
                "xw"|"wx"=>facteur.push(vec![cos,0,0,-sin],
                                        vec![0,1,0,0],
                                        vec![0,0,1,0],
                                        vec![sin,0,0,cos]),
                
                "yz"|"zy"=>facteur.push(vec![1,0,0,0],
                                        vec![0,cos,-sin,0],
                                        vec![0,sin,cos,0],
                                        vec![0,0,0,1]),

                "yw"|"wy"=>facteur.push(vec![1,0,0,0],
                                        vec![0,cos,0,-sin],
                                        vec![0,0,1,0],
                                        vec![0,sin,0,cos]),

                "zw"|"wz"=>facteur.push(vec![1,0,0,0],
                                        vec![0,1,0,0],
                                        vec![0,0,cos,-sin],
                                        vec![0,0,sin,cos]),
                _=>ReportError("Plan de rotation incorrect ou incohérent",plan)
            }
                
            return MultiplicationTMatrices(facteur:Vec<Vec<T>>,Entite:Vec<Vec<T>>)
        }
                                 
        pub fn RotationSimple: fn(f32,Vec<Vec<f32>>,&str,Vec<f32>)-> Vec<Vec<f32>>=RotationUnAxes;
                                  
        pub fn Rotation2D<T: std::fmt::Display>(angle:T, Entite:Vec<Vec<T>>,origine:Vec<T>)->Vec<Vec<T>>{
            if origine.len()!=4{ ReportError("Nombre de dimensions incorrect à l'origine de rotation",origine)}
            let const facteur:Vec<Vec<T>>=vec![ vec![Trigo::cos(angle),-Trigo::sin(angle),0,0],
                                                vec![Trigo::sin(angle),Trigo::cos(angle),0,0],
                                                vec![0,0,1,0],
                                                vec![0,0,0,1]];
            return MultiplicationTMatrices(facteur:Vec<Vec<T>>,Entite:Vec<Vec<T>>)
        }
    }
    pub mod Quaternion{
        pub fn RotationUnAxes<T>(angle:T,Entite:Vec<T>,plan:&str,origine:Vec<T>)->Vec<Vec<T>){
            let quaternion_operator:f32=
        }
    }
    pub mod Entity{
        pub fn TranslationLineaire(mut mesure:Vec<f32>,objet:Entity){
            //Ajoute une quantité vectorielle de déplacement sur toute l'entité
            if json.Debugging==true{ assert_eq(mesure.len(),4);}
            else if mesure.len()!=4{
                for i in mesure.len()..4){
                    mesure.push(0.0);    
            } }
            fn BoucleDereference(Objets){
                let SousStructures:Option<Vec<String>>=dereferencer(Objets);//Méthode
                for objet in SousStructures{
                   //Déférencer en boucle jusqu'aux points
                   if objet.reference==None{
                       if objet.x !=None{
                            //C'est un point=>Changer coordonnées
                            objet.changer_x(point.x+mesure[0]);
                            objet.changer_y(point.y+mesure[1]);
                            objet.changer_z(point.z+mesure[2]);
                            objet.changer_w(point.w+mesure[3]);
                        }
                        else{
                            //C'est une entité référencée d'une certaine façon
                            BoucleDereference(objet.objets);
                        }
                   }
                    else{
                        //Il y a des sous-structures
                        BoucleDereference(objet.reference);
                   }
               }
            }
            
            if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
            else {
                //Passer à travers les références de l'entité
                BoucleDereference(Entite.objets);
                if json.Debugging==true{print!("Translation de l'entité {} de {}{} dans l'axe {}",Entite, mesure, json.Unité, axe);}
            }
        }
        
        fn TranslationUnAxe(mesure:f32,Entite:Entity,axe:String){
            //Ajoute une quantité scalaire de translation dans un seul axe sur toute l'entité
            fn BoucleDereference(Objets){
                let SousStructures:Option<Vec<String>>=dereferencer(Objets);//Méthode
                for objet in SousStructures{
                   //Déférencer en boucle jusqu'aux points
                   if objet.reference==None{
                       if objet.x !=None{
                            //C'est un point
                            match axe{
                               "x"=>objet.changer_x(objet.x+mesure),
                               "y"=>objet.changer_y(objet.y+mesure),
                               "z"=>objet.changer_z(objet.z+mesure),
                               "w"=>objet.changer_w(objet.w+mesure),
                               _=>ReportError("Axe de translation non-existant",axe),
                               None=>panik!("Axe de translation incohérent",axe),
                            }
                        }
                        else{
                            //C'est une entité référencée d'une certaine façon
                            BoucleDereference(objet.objets);
                        }
                   }
                    else{
                        //Il y a des sous-structures
                        BoucleDereference(objet.reference);
                   }
               }
            }
            
            if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
            else {
                //Passer à travers les références de l'entité
                BoucleDereference(Entite.objets);
                if json.Debugging==true{print!("Translation de l'entité {} de {}{} dans l'axe {}",Entite, mesure, json.Unité, axe);}
            }
        }
        fn Rotation(angle:f16,Entite:Entity,axe:String,origine:Vec<f32>){
            //Applique une rotation matricielle des coordonnées selon un axe sur toute l'entité
            //https://en.wikipedia.org/wiki/Rotation_matrix
            fn BoucleDereference(Objets){
                let SousStructures:Option<Vec<String>>=dereferencer(Objets);//Méthode
                for objet in SousStructures{
                   //Déférencer en boucle jusqu'aux points
                   if objet.reference==None{
                       if objet.x !=None{
                            //C'est un point
                           let mut point=vec![objet.x,objet.y,objet.z,objet.w];
                           if 
                            match axe{
                               "x"=>//à CORRIGER LES MESURES
                               "y"=>objet.changer_y(objet.y+mesure),
                               "z"=>objet.changer_z(objet.z+mesure),
                               "w"=>objet.changer_w(objet.w+mesure),
                               _=>ReportError("Axe de rotation non-existant",axe),
                               None=>panik!("Axe de rotation incohérent",axe),
                            }
                            objet.changer_x
                        }
                        else{
                            //C'est une entité référencée d'une certaine façon
                            BoucleDereference(objet.objets);
                        }
                   }
                    else{
                        //Il y a des sous-structures
                        BoucleDereference(objet.reference);
                   }
               }
            }
            
            if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
            else {
                //Passer à travers les références de l'entité
                BoucleDereference(Entite.objets);
                if json.Debugging==true{print!("Rotation de l'entité {} de {}{} dans l'axe {}",Entite, angle, json.Unité, axe);}
            }
        }
    }
}

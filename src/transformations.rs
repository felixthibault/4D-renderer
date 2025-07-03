//! # Fonctions des transformations matriciels des objets.
//! 
//! J'ai séparé la création d'une entité en plusieurs instances d'objets qui sont des
//! points, lignes, carrés, cubes, tesseracts. Chaque instance a sa propre structure 
//! et l'intégration de toute les instances donne l'entité.

#[cfg(not(target_arch = "wasm32"))]
use std::*;
use std::{collections::HashMap, fs::*, io::prelude::*, os::unix::fs::FileExt, string::*};
use bevy::{prelude::*,render::{render_asset::RenderAssetUsages,render_resource::{
    Extent3d, TextureDimension, TextureFormat},},};

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
    //Afficher fenêtre contenant erreur mineure
    //Pour l'instant:
    println!("{} {}.",message,code);
}
pub fn panik() {
    println("cras ans burn");
    panic!("crash and burn");
}

pub fn unreachable(x: Void) -> ! {
    match x {}
}

pub unsafe fn do_nothing(_:Void) -> !{
    return Void
}

fn deferencer(reference:String)->Option<Vec<String>>{
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

fn ScalableFloatMatrix(scalaire:f32,mut matrice:Vec<Vec<f32>>)->Vec<Vec<f32>>{
    //Multiplication d'une matrice par un scalaire float. Retourne une matrice de même dimension. Panique si les types ne sont pas tous des float.
    for mut j in &mut matrice{
        for i in 0..j.len(){
        j[i]*=scalaire;
        }
    }
    return matrice
}

fn ScalableIntMatrix(scalaire:usize,mut matrice:Vec<Vec<usize>>)->Vec<Vec<usize>>{
    //Multiplication d'une matrice d'entiers par un scalaire int non signé. Retourne une matrice de même dimension. Panique si les types ne sont pas tous des usize.
    for mut j in &mut matrice{
        for i in 0..j.len(){
        j[i]*=scalaire;
        }
    }
    matrice//return
}

pub(super) fn MultiplicationFloatMatrices(matrice1:Vec<Vec<f32>>,matrice2:Vec<Vec<f32>>)->Vec<Vec<f32>>{
    //Multiplie des matrices f32 de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    //https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-operations-sur-les-matrices-m1467#multiplication
    if matrice1[0].len()!=matrice2.len(){
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
                calcul+=matrice1[j][case]*matrice2[case][i];
            matrice3[j].push(calcul);
            }    
        }
    }
    return matrice3
}

pub(super) fn MultiplicationIntMatrices(matrice1:Vec<Vec<usize>>,matrice2:Vec<Vec<usize>>)->Vec<Vec<usize>>{
    //Multiplie des matrices d'uX de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    //https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-operations-sur-les-matrices-m1467#multiplication
    if matrice1[0].len()!=matrice2.len(){
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

pub(super) fn MultiplicationTMatrices<T>(matrice1:Vec<Vec<T>>,matrice2:Vec<Vec<T>>)->Vec<Vec<T>>{
    //Multiplie des matrices d'unités inconnues de longueurs quelconques ensemble. Retourne matrice1*matrice2. Panique abruptement si les dimensions ne correspondent pas.
    //https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-operations-sur-les-matrices-m1467#multiplication
    if matrice1[0].len()!=matrice2.len(){
        print("Multiplication de matrices incompatibles");
        println!("Longueur de matrice 1:{}, hauteur de matrice 2:{}",matrice1[0].len(),matrice2.len());
        panik();
    }
    let mut matrice3:Vec<Vec<T>>=Vec::new();
    let mut calcul:T;
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

pub(super) fn completer_matrice<T>(mut matrice:Vec<Vec<T>>,longueur:i16)->Vec<Vec<T>>{
    for i in 0..longueur-matrice.len(){
        matrice.push(vec![0;4]);
    }
    matrice
}

//'! Transformation des points
/* Un matrice de dimension n est une suite de x vecteurs que l'on peut appliquer une transformation.
Un point peut aussi être une matrice de coordonnées (4 lignes et une colonne).
Les matrices peuvent se multiplier entre elles pour former une nouvelle matrice de dimension différente issue d'une transformation. Accumuler ces transformations pour les modification de croquis.
Pour une rotation ou d'autres transformations similaires, transformer d'abord le point en matrice est pratique.
https://web.archive.org/web/20091027131421/http://geocities.com/evilsnack/matrix.htm
https://en.wikipedia.org/wiki/Transformation_matrix
https://www.alloprof.qc.ca/fr/eleves/bv/mathematiques/les-matrices-de-transformation-m1432
https://bibnum.publimath.fr/IST/IST83028.pdf
*/
    
fn Rotation(angle:f16,Entite:Entity,axe:String,origine:Vec<f32>){
    //Applique une rotation matricielle des coordonnées selon un axe sur toute l'entité
    //https://en.wikipedia.org/wiki/Rotation_matrix
    fn BoucleDeference(Objets){
        let SousStructures:Option<Vec<String>>=deferencer(Objets);//Méthode
        for objet in SousStructures{
           //Déférencer en boucle jusqu'aux points
           if objet.reference==None{
               if objet.x !=None{
                    //C'est un point
                   let mut point=vec![objet.x,objet.y,objet.z,objet.w];
                   if 
                    match axe{
                       "x"=>
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
                    BoucleDeference(objet.objets);
                }
           }
            else{
                //Il y a des sous-structures
                BoucleDeference(objet.reference);
           }
       }
    }
    
    if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
    else {
        //Passer à travers les références de l'entité
        BoucleDeference(Entite.objets);
        if json.Debugging==true{print!("Rotation de l'entité {} de {}{} dans l'axe {}",Entite, angle, json.Unité, axe);}
    }
}

}

fn TranslationLineaire(mut mesure:Vec<f32>,objet:Entity){
    //Ajoute une quantité vectorielle de déplacement sur toute l'entité
    if json.Debugging==true{ assert_eq(mesure.len(),4);}
    else if mesure.len()!=4{
        for i in mesure.len()..4){
            mesure.push(0.0);    
    } }
    fn BoucleDeference(Objets){
        let SousStructures:Option<Vec<String>>=deferencer(Objets);//Méthode
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
                    BoucleDeference(objet.objets);
                }
           }
            else{
                //Il y a des sous-structures
                BoucleDeference(objet.reference);
           }
       }
    }
    
    if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
    else {
        //Passer à travers les références de l'entité
        BoucleDeference(Entite.objets);
        if json.Debugging==true{print!("Translation de l'entité {} de {}{} dans l'axe {}",Entite, mesure, json.Unité, axe);}
    }
}


fn TranslationUnAxe(mesure:f32,Entite:Entity,axe:String){
    //Ajoute une quantité scalaire de translation dans un seul axe sur toute l'entité
    fn BoucleDeference(Objets){
        let SousStructures:Option<Vec<String>>=deferencer(Objets);//Méthode
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
                    BoucleDeference(objet.objets);
                }
           }
            else{
                //Il y a des sous-structures
                BoucleDeference(objet.reference);
           }
       }
    }
    
    if Entite.objets==None{ReportError("Aucune référence trouvée dans l'entité",format!("{:?}",Entite));}
    else {
        //Passer à travers les références de l'entité
        BoucleDeference(Entite.objets);
        if json.Debugging==true{print!("Translation de l'entité {} de {}{} dans l'axe {}",Entite, mesure, json.Unité, axe);}
    }
}

fn StretchingInt(mesure:Vec<usize>, matrice:Vec<Vec<usize>>){
    let facteur:Vec<Vec<usize>>=vec![vec![mesure[0],0,0,0],[vec![mesure[1],0,0,0]],[vec![mesure[2],0,0,0]],[vec![mesure[3],0,0,0]]];
}

fn Transformation<S>(){
    //Pour l'instant essayer d'avoir en argument des matrices ou sclalaires d'int ou float. Par la suite voir si les entités sont nécessaires en arguments.
    fn Stretching<T>(mesure:Vec<T>, matrice:Vec<Vec<T>>){
        //Le facteur de mise à l'échelle doit lui-même être une matrice de longueur entre 1 et 4.
        let mut facteur:Vec<Vec<T>>=Vec::new();
        for dimension in 0..mesure.len(){
            facteur.push(vec![0;4]);
            facteur[dimension][dimension]=mesure[dimension];
        }
        if mesure.len()<4{
            //Ajouter des zéros pour compléter en carré
            const facteur=completer_matrice(facteur,4);
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
    MultiplicationTMatrices(facteur:Vec<Vec<T>>,matrice:Vec<Vec<T>>)
    }
}

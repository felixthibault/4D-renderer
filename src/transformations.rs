//! # Fonctions des transformations matriciels des objets.
//! 
//! J'ai séparé la création d'une entité en plusieurs instances d'objets qui sont des
//! points, lignes, carrés, cubes, tesseracts. Chaque instance a sa propre structure 
//! et l'intégration de toute les instances donne l'entité.

#[cfg(not(target_arch = "wasm32"))]

use std::ops::{Add,Mul};
use bevy::{prelude::*,render::{render_asset::RenderAssetUsages,render_resource::{
    Extent3d, TextureDimension, TextureFormat},},};
use Option::Some;
use num_traits::{ToPrimitive,Zero,pow};
    


pub fn test(){
    //Si le test fonctionne, c'est que la fonction est bien appelée.
    println("Module 'transformation' appelé, fonctionnel: Oui");
}

//Fonctions pour aider la production
pub fn println(msg:&str){
    println!("{}",msg);
    //Exemple d'utilisation:
    //print("Oh my glob!");
    //=> Oh my glob!
    //=>
}
pub fn print(msg:&str){
    print!("{}",msg);
    //Exemple d'utilisation:
    //print("Oh my glob!");
    //=> Oh my glob!
}
pub fn report_error(message:&str,code:&str){
    //Afficher fenêtre contenant une erreur mineure
    //Pour l'instant:
    println!("{} {}.",message,code);
}


#[warn(unconditional_recursion)]
pub fn unreachable(){
    type Unreachable=();
    let _x:Unreachable=unreachable();
    unreachable!()//Ceci n'est pas ateignable
}

pub fn not_implemented(){
    todo!();//This will panic
}

pub fn panik() {
    println("crash ans burn");
    panic!("crash and burn");
}

pub unsafe fn do_nothing(_:()) -> (){
    return ()//Fonction doit être appelé avec unsafe{do_nothing};
}

pub fn is_float<S>(x:S)->bool   where S:Into<f64> + Copy, f64: From<S>, {
    //Méthodes de vérification si generic.type ==float or integer. Prend des integers ou floats en paramètres.
    let y= f64::from(x).fract()!=0.0; //Méthode 1
    //let y= f64::from(x)!=i32::from(x) as f64; //Méthode 2, less fonctionnal because i32: From<S> not always implemented.
    return y
}

pub mod convert{
    pub fn convert_f64_to_i64(x: f64) -> Option<i64> {
        let y = x as i64;
        if y as f64 == x {
            Some(y)
        } else {
            None
        }
    }
    pub fn convert_f32_to_i32<T:Into<f32> + Copy>(x: T) -> Option<i32> {
        let y = x.into() as f32 as i32;
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
    pub fn convert_vec<T, U>(vector: Vec<T>) -> Vec<U>
        where T: TryInto<U>, 
        <T as std::convert::TryInto<U>>::Error: std::fmt::Display {
        // Try to convert `Vec<T>` to `Vec<U>`. Mentionner avant l'appel quel sera le type inféré.
        // Créé par Own_Sentence_6928 sur Reddit:https://www.reddit.com/r/learnrust/comments/11hyu0o/help_me_with_making_a_general_function_to_convert/
        // Exemple utilisation: let y:Vec<i16>=convert_vec(vec![7,8,9]);print!("{:?}",y);
        vector
            .into_iter()
            .map(|value_t|match TryInto::try_into(value_t) {
                    Ok(value_u) => value_u,
                    Err(why) => {
                        let t = std::any::type_name::<T>();
                        let u = std::any::type_name::<U>();
                        panic!("Error converting from {t} to {u}: {why}")
                    }
                }
            )
            .collect()
    }
    pub fn convert_matrix<T,U>(matrix:Vec<Vec<T>>) -> Vec<Vec<U>>
        where T: TryInto<U>, 
        <T as std::convert::TryInto<U>>::Error: std::fmt::Display {
        //Try to convert `Vec<<Vec<T>>` to `Vec<Vec<U>>`. Mentionner avant l'appel quel sera le type inféré.
        //Inspiré par la fonction convert_vec citée plus haut.
        //Exemple utilisation: let x:Vec<Vec<i16>>=convert_matrix(vec![vec![7,8,9];4]);print!("{:?}",y);
        matrix
            .into_iter()
            .map(convert_vec)
            .collect()
    }
}


//Fin des fonctions

/*
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
*/

/* 
pub mod  Matrix{
    //inspiration: https://github.com/IgorSusmelj/rustynum/blob/main/docs/tutorials/better-matrix-operations.md
    fn ScalableFloatMatrix(scalaire:f32,mut matrice:&[&[f32]]){
        //Multiplication d'une matrice par un scalaire float. Panique si les types ne sont pas tous des float.
        for mut j in &mut matrice{
            for i in 0..j.len(){
                j[i]*=scalaire;
            }
        }
    }
    
    fn ScalableIntMatrix(scalaire:i32,mut matrice:&[&[usize]]){
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
    
    pub fn addition_slices<T>(a:&[T], b:&[T])->Result<Vec<T>> where T: Add<Output=T>,{
        //Retourne l'addition de deux slices dans un vecteur de même longueur. Retourne une erreur si les longueurs ne correspondent pas. Peut prendre des vecteurs vides.
        if  a.len()!=b.len(){
            if a.is_empty(){ Ok(b) } else if b.is_empty() { Ok(a) }
            print!("Addition de vecteurs incompatibles");
            println!("Longueur de vecteur 1:{}, longueur de vecteur 2:{}",a.len(),b.len());
            return Err("Vecteurs de dimensions incompatibles")
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
        } else if b.is_empty(){ return Ok(Vec::new())} //Les deux sont vides
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

    pub fn transpose<T>(&mut matrice:Vec<Vec<T>>){
        let mut transpose:Vec<Vec<T>>=vec![[0;matrice.len()];matrice[0].len()]; //On crée une nouvelle matrice vide transposée
        transpose.into_iter().map().collect();
        for i in transpose.len(){
            for j in transpose[0].len(){
                transpose[i][j]=matrice[j][i];
            }
        }
        /*
        transpose(&mut matrice);
        pub fn transpose<T>(matrice:&mut Vec<Vec<T>>){
            let mut transpose=vec![Vec::new();matrice[0].len()]; //On crée une nouvelle matrice vide transposée
            //transpose.into_iter().map().collect();
            for i in 0..matrice.len(){
                for j in 0..matrice[i].len(){
                    transpose[i].push(matrice[j][i]);
                }
            }
            *matrice=transpose;
            print!("done");  
        }
        */
    } 
}

pub mod Trigo{
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
*/

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
    

/* 
pub(super) mod Transformation{
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
        
        pub fn translation<S,T>(mut mesure:Vec<S>, mut Entite:Vec<Vec<T>>)->Result<Vec<Vec<T>>>
        where T: Into<f64> + Add<Output=T> + Mul<Output=T> + Clone + num::zero,
              S: Into<f64> + Add<Output=S> + Mul<Output=S> + num::zero,
              f64: From<S>+From<T> {
            //Fonction effectuant une translation sur une matrice de points vectoriels (donc des points étalés dans un vecteur à la verticale et les points sont alignés en longueurs).
            //La matrice de translation est 5x5 donc Entite.len()==4
            //if type de T ==float or T==integer and S==float->renvoyer f64 sinon-> renvoyer type T
            if mesure.len()<4{
                //Manque de dimensions, ajouter des 0
                for dimension in 0..4-mesure.len(){mesure.push(S::zero());}
            } else if mesure.len()>4{
                //Ce CAD est limité à 4 dimension calmdown!
                while mesure.len()!=4{mesure.pop();}
            }

            if is_float(Entite[0][0]) || !is_float(Entite[0][0]) && is_float(mesure[0]){
                //c'est des float
                let facteur:Vec<_>=vec![ vec![1f64,0f64,0f64,0f64,f64::from(mesure[0])],
                                         vec![0f64,1f64,0f64,0f64,f64::from(mesure[1])],
                                         vec![0f64,0f64,1f64,0f64,f64::from(mesure[2])],
                                         vec![0f64,0f64,0f64,1f64,f64::from(mesure[3])],
                                         vec![0f64,0f64,0f64,0f64,1f64] ];
                for mut slice in mut Entite{
                    let slice:Vec<f64>=convert_vec(slice);
                let a:Vec<f64>=vec![1.;Entite[0].len()];
                Entite.push(a);
                }
            } else {
                //c'est des entiers
                let facteur:Vec<_>=vec![ vec![1,0,0,0,T::from(mesure[0])],
                                         vec![0,1,0,0,T::from(mesure[1])],
                                         vec![0,0,1,0,T::from(mesure[2])],
                                         vec![0,0,0,01,T::from(mesure[3])],
                                         vec![0,0,0,0,1] ];
                //Entite reste dans son état
                let a:Vec<T>=vec![1;Entite[0].len()];
                Entite.push(a);
            }
            let Entite=multiplication_matrices(facteur,Entite);
            Entite.pop();//Le dernier vecteur de 1
            return Ok(Entite)
        }
        
        pub fn stretching<T>(mesure:Vec<T>, Entite:Vec<Vec<T>>)->Vec<Vec<T>>{
            //Le facteur de mise à l'échelle doit lui-même être une matrice de longueur entre 1 et 4.
            let mut facteur:Vec<Vec<T>>=Vec::new();
            for dimension in 0..mesure.len(){
                facteur.push(vec![T::zero();4]);
                facteur[dimension][dimension]=mesure[dimension];
            }
            if mesure.len()<4{
                //Ajouter des zéros pour compléter en carré
                for dimension in 0..4-mesure.len(){facteur.push(vec![S::zero();4]);}
            }
            else if mesure.len()==4{
                //On fait f*ckall c'est déjà bon
                do_nothing()
            }
            else if mesure.len()>=4{
                //Trop de dimensions ajoutées dans cette mise en échelle.
                while mesure.len()!=4{mesure.pop();}
            } else {
                //Math is not mathing, please what is going on
                panik();
            }
            return multiplication_matrices(facteur,Entite)
        }
        
        pub fn reflexion_point<T>(origine:Vec<T>, mut Entite:Vec<Vec<T>>)->Vec<Vec<T>>{
            //Effectue la réflexion d'une entité (matrice de points) autour d'un origine de réflexion (point). Retourne une matrice de même dimension.
            if is_float(Entite[0][0]){
                for mut dimension in 0..Entite.len(){
                    for mut point in mut Entite[dimension]{
                        let point:T=2.*origine[dimension]-point;
                    }
                }
            } else {
                for mut dimension in 0..Entite.len(){
                    for mut point in mut Entite[dimension]{
                        let point:T=2*origine[dimension]-point;
                    }
                }
            }
            return Entite    
        }
        /*
        pub fn reflexion<T>(A:T, B:T, C:T, D:T, E:T, &Entite:&[&[T]])->Vec<Vec<f32>> 
        where T: Into<f32> + Add<Output=T> + Mul<Output=T>,
              f32: From<T> {
            //Effectue la réflexion d'une entité (matrice de points) autour d'un objet à n dimension, préférablement entre 1 et 3 dimensions (ligne, plan ou volume). Retourne une matrice de f32 de même dimension.
            //Le nombre de points d'origine de réflexion possibles fait partie des réels^n. Si n=0 il y a un seul point de réflexion et non une infinité.
            //L'espace de réflexion est donné par la règle Ax+By+Cz+Dw+E=0 et se comporte comme un plan en 3D, mais le volume délimite deux tranches de la 4D.
            //Les formules ci-dessous sont simulées dans desmos. 
                réflexion 1D: https://www.desmos.com/calculator/5z9m4uiqjt?lang=fr
                réflexion 2D: https://www.desmos.com/calculator/x0ru3lts4o?lang=fr
                réflexion 3D: https://www.desmos.com/3d/xtwdh9l3tc?lang=fr
                réflexion 4D: https://www.desmos.com/3d/pdnpl6cqre?lang=fr
            
            let longueur_matrice:u16=Entite[0].len();
            let mut image_entite:Vec<Vec<f32>>=vec![vec![f32::zero();longueur_matrice];4];
            for point in 0..longueur_matrice{
                //Modifier chacun des points pour obtenir le point image.
                image_entite[0][point]=x(&Entite[0][point],&Entite[1][point],&Entite[2][point],&Entite[3][point]);
                image_entite[1][point]=y(&Entite[0][point],&Entite[1][point],&Entite[2][point],&Entite[3][point]);
                image_entite[2][point]=z(&Entite[0][point],&Entite[1][point],&Entite[2][point],&Entite[3][point]);
                image_entite[3][point]=w(&Entite[0][point],&Entite[1][point],&Entite[2][point],&Entite[3][point]);
            }
            fn x<T>(alpha:T, phi:T,theta:T, beta:T)->&f32{
                if is_float(alpha){
                    ((-pow(A,2)*alpha+pow(B,2)*alpha+pow(C,2)*alpha+pow(D,2)*alpha-2.*A*B*phi-2.*A*C*theta-2.*A*D*beta-2.*A*E)/(pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2))) as f32
                } else {
                    f32::from((-pow(A,2)*alpha+pow(B,2)*alpha+pow(C,2)*alpha+pow(D,2)*alpha-2*A*B*phi-2*A*C*theta-2*A*D*beta-2*A*E))/f32::from((pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2)))
                }  
            }
            fn y<T>(alpha:T, phi:T,theta:T, beta:T)->&f32{
                if is_float(alpha){
                    ((pow(A,2)*phi-pow(B,2)*phi+pow(C,2)*phi+pow(D,2)*phi-2.*B*A*alpha-2.*B*C*theta-2.*B*D*beta-2.*B*E)/(pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2))) as f32
                } else {
                    f32::from((pow(A,2)*phi-pow(B,2)*phi+pow(C,2)*phi+pow(D,2)*phi-2*B*A*alpha-2*B*C*theta-2*B*D*beta-2*B*E))/f32::from((pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2)))
                }
            }
            fn z<T>(alpha:T, phi:T,theta:T, beta:T)->&f32{
                if is_float(alpha){
                    ((pow(A,2)*theta+pow(B,2)*theta-pow(C,2)*theta+pow(D,2)*theta-2.*C*A*alpha-2.*C*B*phi-2.*C*D*beta-2.*C*E)/(pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2))) as f32
                } else {
                    f32::from((pow(A,2)*theta+pow(B,2)*theta-pow(C,2)*theta+pow(D,2)*theta-2*C*A*alpha-2*C*B*phi-2*C*D*beta-2*C*E))/f32::from((pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2)))
                }
            }
            fn w<T>(alpha:T, phi:T,theta:T, beta:T)->&f32{
                if is_float(alpha){
                    ((pow(A,2)*beta+pow(B,2)*beta+pow(C,2)*beta-pow(D,2)*beta-2.*D*A*alpha-2.*D*B*phi-2.*D*C*theta-2.*D*E)/(pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2))) as f32
                } else {
                    f32::from((pow(A,2)*beta+pow(B,2)*beta+pow(C,2)*beta-pow(D,2)*beta-2*D*A*alpha-2*D*B*phi-2*D*C*theta-2*D*E))/f32::from((pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2)))
                }
            }
            return image_entite
        }
        */
        pub fn reflexion<T>(A:T, B:T, C:T, D:T, E:T, &Entite:Vec<Vec<T>>)->Result<Vec<Vec<f32>>>
            where T: Into<f32>, Add<Output=T> + Mul<Output=T>,
                f32: From<T>{
            //Effectue la réflexion d'une entité (matrice de points) autour d'un objet à n dimension, préférablement entre 1 et 3 dimensions (ligne, plan ou volume). Retourne une matrice de même dimension.
            //Retourne une erreur si A²+B²+C²+D²≠1.
            //Le nombre de points d'origine de réflexion possibles ∈ ℝ^n (réels). Si n=0 il y a un seul point de réflexion et non une ou plusieurs infinités.
            //L'espace de réflexion est donné par la règle Ax+By+Cz+Dw+E=0 et se comporte comme un plan en 3D, mais le volume délimite deux tranches de la 4D.
            //Les formules ci-dessous sont simulées dans desmos. 
            /*  réflexion 1D: https://www.desmos.com/calculator/5z9m4uiqjt?lang=fr
                réflexion 2D: https://www.desmos.com/calculator/x0ru3lts4o?lang=fr
                réflexion 3D: https://www.desmos.com/3d/xtwdh9l3tc?lang=fr
                réflexion 4D: https://www.desmos.com/3d/pdnpl6cqre?lang=fr
            */
            if f32::from(pow(A,2)+pow(B,2)+pow(C,2)+pow(D,2)) != 1f32{ return Err("Reçu vecteur non unitaire");}
            let mut image_entite:Vec<Vec<T>>=Entite; 
            if Entite.len()<4{
                //Manque de dimension dans cette réflexion
                for dimension in 0..4-Entite.len(){image_entite.push(vec![T::zero();image_entite[0].len()]);}
            } else if Entite.len()>4{
                //Trop de dimension. Surement une erreur, le surplus sera tronqué
                while image_entite.len()!=4{image_entite.pop();}
            } else {
                //On fait f*ckall c'est déjà bon
                do_nothing()
            } 
            assert_eq(4, image_entite.len());
            if is_float(image_entite[0][0]){
                image_entite.push(vec![1.;image_entite[0].len()]);
                let facteur:Vec<Vec<T>>= vec![ vec![1.-2.*pow(A,2),-2.*A*B,-2.*A*C,-2.*A*D,-2.*A*E],
                                               vec![1.-2.*pow(B,2),-2.*B*A,-2.*B*C,-2.*B*D,-2.*B*E],
                                               vec![1.-2.*pow(C,2),-2.*C*A,-2.*C*B,-2.*C*D,-2.*C*E],
                                               vec![1.-2.*pow(A,2),-2.*D*A,-2.*D*B,-2.*D*C,-2.*D*E],
                                               vec![T::zero(),T::zero(),T::zero(),T::zero(),1.] ];
            } else{
                image_entite.push(vec![1;image_entite[0].len()]);
                let facteur:Vec<Vec<T>>= vec![ vec![1-2*pow(A,2),-2*A*B,-2*A*C,-2*A*D,-2*A*E],
                                               vec![1-2*pow(B,2),-2*B*A,-2*B*C,-2*B*D,-2*B*E],
                                               vec![1-2*pow(C,2),-2*C*A,-2*C*B,-2*C*D,-2*C*E],
                                               vec![1-2*pow(A,2),-2*D*A,-2*D*B,-2*D*C,-2*D*E],
                                               vec![T::zero(),T::zero(),T::zero(),T::zero(),1] ];
            }

            let image_entite=multiplication_matrices(facteur,image_entite);
            return Ok(image_entite.pop())//Enlève le surplus de 1 ajouté
        }
        
        pub fn homothetie<T>(k:T, &mut Entite:Vec<Vec<T>>, &origine:&[T])->Result<Vec<Vec<T>>>{
            //Effectue une homothétie (agrandissement ou réduction) d'une entité. Retourne une matrice de même dimension.
            //Une homothétie transforme un vecteur SX'->kSX où k est le rapport d'homothétie, S le centre d'homothétie et X le point à déplacer.
            //Les rapports d'angles, de segments et des relations dans l'entité sont préservés par l'homothétie, et ce dans n'importe quelle dimension.  
            //https://en.wikipedia.org/wiki/Homothety
            let facteur:Vec<Vec<T>>= vec![  vec![k, T::zero(), T::zero(), T::zero(), (1-k)*origine[0]],
                                            vec![T::zero(), k, T::zero(), T::zero(), (1-k)*origine[1]],
                                            vec![T::zero(), k, T::zero(), T::zero(), (1-k)*origine[2]],
                                            vec![T::zero(), k, T::zero(), T::zero(), (1-k)*origine[3]],
                                            vec![T::zero(), k, T::zero(), T::zero(), T::from(1)],];
            Entite.push(vec![T::from(1);Entite[0].len()])//Ajoute des 1 à la fin
            Ok(multiplication_matrices(facteur, Entite).pop());
        }
        
        pub fn rotation_2d<T>(theta:T, Entite:Vec<Vec<T>>, &origine:&[T])->Result<Vec<Vec<f32>>>
            where T: Into<f32> + Add<Output=T> + Mul<Output=T>,
                f32: From<T>{
            //Effectue la rotation d'une entité 2D autour d'un point (x,y). 
            //Prend des angles en radians. Fonction ne devrait pas être appelée dans ce cad puisque limitée au 2d.
            //Si le x du point à tourner est < à origine[0], ajouter +pi à thêta.
            let &A:T=origine[0]; let &B:T=origine[1];
            let mut image_entite:Vec<Vec<f32>>=vec![vec![0f32;Entite[0].len()];2]
            for i in 0..Entite[0].len(){
                if Entite[0][i]<A{
                    image_entite[0][i]=(f32::from(pow(Entite[0][i]-A,2)+pow(Entite[1][i]-B,2))).sqrt()*Trigo::cos(theta+Trigo::arctan(f32::from(Entite[1][i]-B)/f32::from(Entite[0][i]-A)));
                    image_entite[1][i]=(f32::from(pow(Entite[0][i]-A,2)+pow(Entite[1][i]-B,2))).sqrt()*Trigo::sin(theta+Trigo::arctan(f32::from(Entite[1][i]-B)/f32::from(Entite[0][i]-A)));
                } else {
                    image_entite[0][i]=(f32::from(pow(Entite[0][i]-A,2)+pow(Entite[1][i]-B,2))).sqrt()*Trigo::cos(
                        theta+3.141592653589793238462643383f32+Trigo::arctan(f32::from(Entite[1][i]-B)/f32::from(Entite[0][i]-A)));
                    image_entite[1][i]=(f32::from(pow(Entite[0][i]-A,2)+pow(Entite[1][i]-B,2))).sqrt()*Trigo::sin(
                        theta+3.141592653589793238462643383f32+Trigo::arctan(f32::from(Entite[1][i]-B)/f32::from(Entite[0][i]-A)));
                }
            }
            return Ok(image_entite)
        }
        pub fn rotation_2d_mieux<T>(theta:T, &Entite:Vec<Vec<T>>, &origine:&[T])->Result<Vec<Vec<f32>>>
            where T: Into<f32>, f32: From<T>{
            //Effectue la rotation d'une entite 2D autour d'un point (x,y). Prend des angles en radians. Retourne une matrice de mêmes dimensions de f32.
            //Forme plus linéaire et fonctionnelle que rotation_2d. Fonction ne devrait pas être appelée dans ce cad puisque limitée au 2d.
            let A:g32=f32::from(origine[0]); let B:g32=f32::from(origine[1]);
            let facteur:Vec<Vec<f32>>=vec![ vec![Trigo::cos(theta),-Trigo::sin(theta),-A*Trigo::cos(theta)+B*Trigo::sin(theta)+A],
                                            vec![Trigo::sin(theta),Trigo::cos(theta), -A*Trigo::sin(theta)-B*Trigo::cos(theta)+B],
                                            vec![0f32,0f32,1f32] ];
            let mut image_entite:Vec<Vec<f32>>=Convert::convert_matrix(Entite);
            image_entite.push(vec![1f32;Entite[0].len()]);
            Ok(multiplication_matrices(facteur,image_entite).pop())
        }

        pub fn rotation_4d<T>()->Result<Vec<Vec<f32>>>{
            //Effectue la rotation quadridimensionnelle d'une entité selon les angles m1 et m2 sur deux plans orthogonaux.
            //Prend des angles en radians. Retourne une matrice de mêmes dimensions de f32.
            //Se référer au desmos https://www.desmos.com/3d/bweaese70a?lang=fr pour trouver les formules originales.
            //Se référer au fichier Explication rotation 4D.txt pour comprendre l'origine des formules et de la matrice exponentielle.

        }
        pub fn rotation_matrix<T>(angle:T, &vecteur:&[T], &wecteur:&[T])->Result<Vec<Vec<f32>>>
            where T: Into<f32>, f32: From<T>{
            //Renvoie une matrice de rotation réutilisable pour une transformation plus complexe où il y a plusieurs plans et angles.
            //Nécessite un angle et un plan, défini par six variables ou deux vecteurs connus, vector 1 et vector 2.
            //Idéalement envoyer à la fonction génératrice de plan les 6 variables avant d'envoyer les vecteurs ici.
            //Il faut vraiment que le produit scalaire de vecteur et wecteur=0 pour éviter les réponses absurdes...
            I:Vec<Vec<T>=vec![ vec![1,0,0,0],
                               vec![0,1,0,0],
                               vec![0,0,1,0],
                               vec![0,0,0,1], ]; //Matrice identité 4x4 adaptée
            // Formule en texte: R(θ)=e^(θ×(BA^T-AB^T)=I+(AA^T+BB^T)×(cos(θ)-1)+(BA^T-AB^T)×sin(θ)
            I+
            
            
        }
    
    }
    pub mod Quaternion{
        do_nothing(1);
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
*/

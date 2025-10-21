/*
    Fournit une API pour contrôler les transformations matricielles avec le langage Rust.
    Ceci n'est pas une copie conforme, mais de sa source rapprochée 
    de numpy.ndarray de Numeric Python.
    
    Maintenance: felixthibault2007@gmail.com
    
    par/by Félix Thibault 2025

    Ce fichier comprend les fonctions implémentées suivantes:

    Les fonctions qu'il reste à implémenter sont les suivantes: opérations mathématiques de base,
    opérations de systèmes d'équations, objets multidimensionnels.
*/

use array_object::Array;
//! Structure de la classe des matrices compilées en rust
#[derive(Debug)]
pub fn test(){
        print!("test");
    }

//Opérations mathématiques sur les matrices de base non spécifiées. 
//Cette section comporte: l'addition, le produit de deux matrices, le produit d'une 
//série de matrices, la multiplication par un scalaire.
trait TraitAddition{
    ///Ce trait permet de défénir le comportement des fonctions où des types inconnus
    /// sont additionnées et ce qu'ils retournent.
    ///Note: pour des lettres, on peut ajouter un String et un &str, puisque le String contient déjà
    /// la propriété de la modification mémoire des valeurs, mais on ne peut faire &str+&str
    /// sans que un des str soit mis proriétaire avec &str.to_owned()+&str. 
    ///Tous les types additionnés doivent être identiques et sont typiquement des nombres
    /// ou des lettres. 
    pub fn add_matrix(&matrice1:RustArray, &matrice2:RustArray);

}

pub fn add_matrix<H>(&mut matrice1:RustArray<H>, &matrice2:RustArray<H>) -> Option<RustArray<H>> 
    where H:Add, Copy{
    /// Si la matrice 1 n'est pas du même type que la matrice2, on doit convertir de l'un à l'autre.
    /// Par souci de structure fixe, la première matrice est défini pour contenir l'espace de stockage
    /// ou les données sont accumulées. Donc, si les visualisations ne sont pas identiques, la matrice res
    /// restante est convertie pour être lue (précisément ce mot) dans le même type que la matrice1. En 
    /// principe cependant, passer d'une vue en colonne à une vue Fortran est le même procédé que
    /// l'inverse, c'est une simple transpose de mémoire.
    /// ATTENTION: si la forme des deux matrices additionnées n'est pas la même, tout en ayant 
    /// matrice1.data.len()=matrice2.data.len(), ceci est un Undefined_behavior, puisque le
    /// comportement varie selon shape.
    if matrice1.data.len()!=matrice2.data.len(){
        None //Impossibilité fondamentale d'additionner pour éviter des memory leaks
    }
    else {
        //On continue
        //Lorsqu'on fait Vec+Vec, on ne doit pas faire vec.extend()
        let (matrice1_shape,matrice2_shape)=(matrice1.shape,matrice2.shape);
        if matrice1_shape!=matrice2_shape{
            //Vérifier si shape est simplement un None avec un "C"
            if matrice1_shape.unwrap()=="F" || matrice2_shape.unwrap()=="F" {
                //Les matrices n'ont pas la même visualisation, il faut transposer la deuxième.
                //On peut envoyer simplement matrice2.data à to_fortran()
                let copy=
            }
            else{//Rien
            }
        }
        else{//Rien
        }
    }
}
pub fn scalable_matrix(&mut self, scalaire:T) where T:Clone+ One+ Mul, H:Clone+ One+ Mul, {
    //Multiplie une matrice par un scalaire. Ce produit est commutatif.
    //La fonction ne retourne rien pour l'instant. 
    //self and scalaire must be numbers tho.
    let mut x=&mut self.data;
    for donnee in x{
        donnee*=scalaire;
    }
}



pub fn scalable_matrix<T>(&mut self, scalaire:T) where T:Clone+ One+ Mul, H:Clone+ One+ Mul, {
    //Multiplie une matrice par un scalaire. Ce produit est commutatif.
    //La fonction ne retourne rien pour l'instant. 
    //self and scalaire must be numbers tho.
    let mut x=&mut self.data;
    for donnee in x{
        donnee*=scalaire;
    }
}
impl Scalar for 
pub fn produit_matrix
pub fn produit_multi_matrix


//Opérations de Gauss et Élémentaires Lignes pour les matrices équations.
//Cette section comporte: les opérations élémentaires ligne (OEL) la résolution complète
//de matrices, le nombre de pivots, le nombre d'équations, le nombre de variables, 
//l'échelonnage de matrice de Gauss (SEL), l'échelonnage de matrice de Gauss-Jordan (SELH).


//Opérations systèmes pour les matrices de structures comme des objets multidimensionnels.


/*Implémentation d'un trait selon le type:
impl Trait_trait for T{
    pub fn Myfunc(){}
} */
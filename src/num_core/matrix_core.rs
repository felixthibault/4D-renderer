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

/*impl TraitAddition for i32 {
    add_matrix
} */
pub fn add_matrix<H>(&matrice1:RustArray<H>, &matrice2:RustArray<H>) -> Option<RustArray<H>> 
    where H:Add, {
    if matrice1.data.len()!=matrice2.data.len(){None}
    else { let (m1,m2)=(matrice1,matrice2);}
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

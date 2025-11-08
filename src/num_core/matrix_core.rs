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
    pub fn add_matrix(&mut matrice1:RustArray, &matrice2:RustArray);

}

pub fn addition_vec<T>(a:&Vec<T>, b:&Vec<T>)-> Result<Vec<T>, &'static str>
    where T: Add<Output=T>+ Copy {
    ///Retourne l'addition de références de deux vecteurs dans un vecteur référencé 
    /// de même longueur. Retourne une erreur si les longueurs ne correspondent pas. 
    ///Peut prendre des vecteurs vides.
    ///On déduit que puisque cette fonction est une addition, les types implémentent
    /// Copy, comme tous les integers et nombres flottants
    if  a.len()!=b.len(){
        if a.is_empty(){ return Ok(b.clone()); } else if b.is_empty() { return Ok(a.clone()); }
        print!("Addition de vecteurs incompatibles");
        println!("Longueur de vecteur 1:{}, longueur de vecteur 2:{}",a.len(),b.len());
        return Err("Vecteurs de dimensions incompatibles")
    } else if a.is_empty(){ return Ok(a.clone()); } //Les deux sont vides, on retourne au hasard a
    a.iter()//Tout va bien ici
        .zip(b.iter())
        .map(|(&x, &y)| x + y)
        .collect()
}

pub fn add_matrix<H>(&mut matrice1:RustArray<H>, &matrice2:RustArray<H>) -> Option<RustArray<H>> 
    where H: Add<Output=H>+ Copy {
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
        //Lorsqu'on fait Vec+Vec, on ne doit pas faire vec.extend(), puisqu'on ajoute pas des valeurs
        let (matrice1_shape,matrice2_shape)=(&matrice1.view,&matrice2.view);
        if matrice1_shape!=matrice2_shape{//*erreur d'écriture, on parle ici de la visualisation, pas de la forme
            //Vérifier si shape est simplement un None avec un "C"
            if matrice1_shape==Some("F") {
                //Les matrices n'ont pas la même visualisation, il faut transposer la deuxième.
                //On transpose en Fortran style
                let mut copy=*matrice2;
                matrice1.data= *addition_vec(&matrice1.data, &copy.to_fortran().data);
                Some(matrice1)
            }
            else if matrice2_shape==Some("F") {
                //Les matrices n'ont pas la même visualisation, il faut transposer la deuxième.
                //On transpose en Colonne style
                let mut copy=*matrice2;
                ///On envoie crée une copy de matrice2 en déréférençant ses données pour en modifier
                /// la copie en changeant la visualisation, après on additionne la référence à ce
                /// vecteur.
                matrice1.data= *addition_vec(&matrice1.data, &copy.to_column().data);
                Some(matrice1)
            }
            ///Cependant, il faut que l'addition soit aussi déréférencée des vecteurs réfécencés, soit
            /// matrice1 et matrice2 ou copy, puisque les valeurs seront droppées, on ne sait pas 
            /// comment les variables seront modifiées en dehors de la fonction et aussi copy est
            /// temporaire et sera supprimée lorsque droppée.
            else{//Rien
                ///Effectuer l'addition comme prévu, les deux ont une visualisation similaire
                do_nothing();
                None
            }
        }
        else{//Rien
            ///Effectuer l'addition comme prévu, ceci est un undefined_behavior
            do_nothing();
            None
        }
        ///À ce stade, une incompatibilité des formes des matrices est considérée comme un 
        /// undefined_behavior même si le programme peut tout de même rouler.
        matrice1.data= *addition_vec(&matrice1.data, &matrice2.data);
        Some(matrice1)
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
//Référence sur les types: https://doc.rust-lang.org/nightly/reference/special-types-and-traits.html?


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

pub unsafe fn do_nothing(_:Void) -> !{
    return Void
}
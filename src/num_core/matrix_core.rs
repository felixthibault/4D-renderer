/*
    Fournit une API pour contrôler les transformations matricielles avec le langage Rust.
    Ceci n'est pas une copie conforme, mais de sa source rapprochée 
    de numpy.ndarray de Numeric Python.
    
    Maintenance: felixthibault2007@gmail.com
    
    par/by Félix Thibault 2025

    Ce fichier comprend les fonctions implémentées suivantes:

    Les fonctions qu'il reste à implémenter sont les suivantes: résolution de matrice de 
    système d'équations linéaires, transpose, produit_matrices, mise en échelle, rotation 2D, 
    rotation 4D, scalable_matrix, addition_matrices, completer_matrice_carre, sorting_system,
*/

//! Structure de la classe des matrices compilées en rust
#[derive(Debug)]
pub struct matrix{
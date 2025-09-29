/*
    Fournit une API pour créer des objets matriciels avec le langage Rust.
    Ceci n'est pas une copie conforme, mais se rapproche par sa source 
    de numpy.ndarray de Numeric Python et rnp.NumArray de RustyNum.
    
    Maintenance: felixthibault2007@gmail.com
    
    par/by Félix Thibault 2025
*/
use num_traits::Zero;
use num::ParseIntError;

mod Array{
    
    pub struct RustArray<H>{
        /*Construction de l'objet matriciel en rust
        -Arguments nécessaires à la formation d'un array simple: une liste de valeurs de type T
        -Arguments nécessaires à la création d'une matrice complète: array simple + tuple(shape) */
        data: Vec<H>,//J'espère implenter tous les types si possible.
        shape: Option<(usize, usize)>,//La forme d'une matrice est strictement positive, bien que l'argument d'entrée ne le soit pas.
        equation: bool,//La matrice peut représenter un système d'équations linéaires(true) ou un tableau de points(false).
    }
    impl<H,U> RustArray<H,U>{
        pub const fn new() -> RustArray<H> {//Crée une nouvelle matrice vierge, dynamique et sous forme de matrice de points.
            RustArray{
                data: Vec::new(),
                shape: None,
                equation: false,
            }
        }
        pub fn with_capacity(capacite:usize) -> RustArray<H> {
            RustArray{
                data: Vec::with_capacity(capacite),
                shape: None,
                equation: false,
            }
        }
        pub fn zeros<U: num::Zero>(shapes:(usize, usize)) -> RustArray<H,U> {
            //Crée un array de 0 de type U de forme shapes
            let longueur=shapes.0*shapes.1;
            RustArray{
                data: vec![U::zero();longueur],
                shape: shapes,
                equation: false,}
        }
        pub fn reshape(&mut self, Option<shape:(usize, usize)>) -> Result<(), ParseIntError>{
            /*Reforme une matrice unidimensionnelle à une forme multidimensionnelle.
            Si la matrice a déjà une forme=!(x,0), la fonction va vérifier ce qui est possible de modifier.
            Nécessite un tuple de 2 arguments.
            Pour des cas particuliers où une dimension est inconnue, envoyer à , unknown_reshape.
            self: vecteur-style à être reformé
            shape: tuple(usize ou u32)
            */
            if shape==None{Err(-1);}
            else{
                //Reshape et chercher
            }
        }
        pub fn unknown_reshape(&mut self, shape:(isize,isize)) -> Result<(), ParseIntError>{
            /*Reforme une matrice unidimensionnelle ou avec une forme en une matrice multidimensionnelle.
            Puisque un des nombres du tuple est supposé être -1 (donc que sa mesure est inconnue), deux nombres sera refusé.
            Nécessite un tuple de 2 arguments. Si deux nombres>1=>reshape()
            Le programme va essayer de voir quelles combinaisons sont possibles.
            self: vecteur-style à être reformé
            shape: tuple(isize ou i32)
            */
            if shape.0 >=0 && shape.1 >=0 {reshape(&mut self, (shape.0 as usize, shape.1 as usize));}
            if shape.0 ==-1 && shape.1 >=0 || shape.0 >=0 && shape.1 ==-1{
                //Reshape et chercher
                Ok()
            } else {Err(-1);}
        }
        pub fn flatten(&mut self){ 
            //Rend un objet RustArray multidimensionnel à une forme 1D aplatie
            
        }
        pub fn shape(&self) -> Option<(usize, usize)> {
            //Retourne la forme de la matrice RustArray
            self.shape
        }
    }
    
//se référer à numpy.ndarray pour trouver des idées de fonctions: https://numpy.org/doc/1.22/reference/generated/numpy.ndarray.html?
}
/*
mod Reshape{
    pub trait Reshapable{
        fn reshape(self, colonnes:usize, lignes:usize) ->self { 
        }
    }
    impl Reshapable for RustArray{
        pub fn reshape(mut self, colonnes:usize, lignes:usize) ->self {
            /*Reforme une matrice unidimensionnelle à une forme multidimensionnelle
            Nécessite 2 arguments ou un tuple pour des cas particuliers. Si une variable est -1, va vérifier selon contexte si forme possible.
            array: vecteur-style à être reformé
            shape: int(usize ou u32) ou tuple(isize ou i32)
                Si type=tuple, les arguments peuvent être des nombres négatifs pour que le système calcule lui-même la forme optimale
            */
        }
        pub fn flatten(){ //Rend un objet RustArray multidimensionnel à une forme 1D aplatie
        }
    
    }
}
*/
pub fn range(start:i32, stop:i32)->Vec<i32>{ //Retourne une liste (array) de nombre contenant l'intervalle fourni -1
    if start<stop{(start..stop).collect()}
    else{(stop..start).collect()}
}
pub fn big_range(start:&isize, stop:&isize)->Vec<isize>{ //Retourne une liste (array) pour les gros intervalles
    //let mut a=Vec::with_capacity((stop-start).try_into().unwrap_or(1));
    //for i in *start..*stop{a.push(i)}; a
    (*start..*stop).collect()//Version simplifiée
}

pub fn test_exemple(){
    //Fournit une représentation de la façon à utiliser l'objet rnp.RustArray
    let matrix_data = rnp.RustArray(range(0,16)); // 0 to 15
    let matrix= matrix_data.reshape(4,4);
    println!("{}\n{}",matrix_data,matrix);
}
Vec::with_capacity(9)
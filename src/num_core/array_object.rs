/*
    Fournit une API pour créer des objets matriciels avec le langage Rust.
    Ceci n'est pas une copie conforme, mais se rapproche par sa source 
    de numpy.ndarray de Numeric Python et rnp.NumArray de RustyNum.
    
    Maintenance: felixthibault2007@gmail.com
    
    par/by Félix Thibault 2025
*/
#![allow(unused)]
use num_traits::Zero;
use num::ParseIntError;

mod Array{
    enum MatrixKind{
        Eq, //Equation Matrix type
        Trans, //Transformation Matrix type
        Point, //Matrix of points (x,4) where points are placed vertically
        Line, //Matrix of lines (x,2) where 2 references(point) are placed vertically. The number in each square correspond to the position of this point in the matrix of points.
        Plg, //Matrix of polygons (x,y) where each polygon is displaced vertically. The number in each square correspond to the position of the line in the matrix of lines.
        Ple, //Matrix of polyhedrons(polyèdres) (x,y) where each polyhedron is displaced vertically. The number in each square correspond to the position of the polygon in the matrix of polygon.
        Plc, //Matrix of 4-polytope(polychore) (x,y) where each 4-polytope is displaced vertically. The number in each square correspond to the position of the polyhedron in the matrix of polyhedrons.
    }
    pub struct RustArray<H>{
        /*Construction de l'objet matriciel en rust
        -Arguments nécessaires à la formation d'un array simple: une liste de valeurs de type T
        -Arguments nécessaires à la création d'une matrice complète: array simple + tuple(shape) */
        data: Vec<H>,//J'espère implenter tous les types si possible.
        shape: Option<(usize, usize)>,//La forme d'une matrice est strictement positive, bien que l'argument d'entrée ne le soit pas.
        //equation: bool,//La matrice peut représenter un système d'équations linéaires(true) ou un tableau de points(false).
        kind: MatrixKind,// La matrice peut représenter une transformation linéaire, un système d'équations linéaires ou des objets.
    }
    impl<H> RustArray<H>{
        pub const fn new() -> RustArray<H> {
            //Crée une nouvelle matrice vierge, dynamique et sous forme de matrice de transformation.
            RustArray{
                data: Vec::new(),
                shape: None,
                kind: MatrixKind::Trans,
            }
        }
        pub fn with_capacity(capacite:usize) -> RustArray<H> {
            //Crée une nouvelle matrice de transformation vierge, mais avec une capacité prédéfinie si elle est connue
            RustArray{
                data: Vec::with_capacity(capacite),
                shape: None,
                kind: MatrixKind::Trans,
            }
        }
        pub fn zeros(shapes:(usize, usize)) -> RustArray<H> where H: num::Zero + std::clone::Clone + Copy {
            //Crée un array de 0 de type H de forme shapes
            let longueur=shapes.0*shapes.1;
            RustArray{
                data: vec![H::zero();longueur],
                shape: Some(shapes),
                kind: MatrixKind::Trans,}
        }
        pub fn reshape(&mut self, shape:Option<(usize, usize)>) -> Option<shape> {
            /*Reforme une matrice unidimensionnelle à une forme multidimensionnelle.
            Si la matrice a déjà une forme=!(x,0), la fonction va vérifier ce qui est possible de modifier.
            Nécessite un tuple de 2 arguments.
            Pour des cas particuliers où une dimension est inconnue, envoyer à , unknown_reshape.
            self: vecteur-style à être reformé
            shape: tuple(usize ou u32)
            */
            let x=self.data;
            if shape==None || x.len() != shape.unwrap().0*shape.unwrap().1 {
                //Forme est nulle ou longueur du vecteur inadaptée pour le volume de la matrice
                -1
            }
            else{
                self.shape=shape;
                return shape
            }
        }
        pub fn unknown_reshape(&mut self, shape:Option<(isize,isize)>) -> Option<shape> {
            /*Reforme une matrice unidimensionnelle ou avec une forme en une matrice multidimensionnelle.
            Puisque un des nombres du tuple est supposé être -1 (donc que sa mesure est inconnue), deux inconnus sera refusé.
            Nécessite un tuple de 2 arguments. Si deux nombres>1=>reshape()
            Le programme va essayer de voir quelles combinaisons sont possibles.
            self: vecteur-style à être reformé
            shape: tuple(isize ou i32)
            */
            let x=self.data;
            if shape.0 >=0 && shape.1 >=0 {(&mut self).reshape(Some(shape.0 as usize, shape.1 as usize));}
            else if shape.0 ==-1 && shape.1 >=0 {
                //Reshape et chercher longueur
                if shape!=self.shape && (self.data.len() as f32/shape.1 as f32-(self.data.len()/shape.1) as f32)==0.{
                    self.shape=Some( (self.data.len()/shape.1,shape.1) );
                }
                return Some(shape)
            } 
            else if shape.0 >=0 && shape.1 ==-1 {
                //Reshape et chercher hauteur
                if shape!=self.shape && (self.data.len() as f32/shape.0 as f32-(self.data.len()/shape.0) as f32)==0.{
                    self.shape=Some( (shape.0,self.data.len()/shape.0) );
                }
                return Some(shape)
            }
            else {return -1 //aucune combinaison gagnante (mauvais envoi ou deux inconnus}
        }
        pub fn flatten(&mut self){ 
            //Rend un objet RustArray multidimensionnel à une forme 1D aplatie
            self.shape=None;
        }
        pub fn push_col(&mut self, col: Vec<H>)->Option<col>{
            //Pousse une colonne dans un objet RustArray. Retourne une erreur si la colonne ne correspond pas à shape.1
            if col.len()!=self.shape.1{Some(col)}
            //Ajouter la colonne selon la forme de self
            let x=self.shape.1;
            for money in 0..col.len(){
                self.data.insert((money+1)*(x+1),col[money]);
            }
            Some(col)
        }
        pub fn push_col(&mut self, row: Vec<H>)->Option<row>{
            //Pousse une ligne dans un objet RustArray. Retourne une errur si la ligne ne correspond pas à shape.0
            if row.len()!=self.shape.0{Some(row)}
            //Ajouter la ligne selon la forme de self
            self.data.push(row);
            
            Some(col)
        }
        pub fn get_data(&self) -> &Vec<H>{
            &self.data
        }
        pub fn get_shape(&self) -> &Option<(usize, usize)> {
            //Retourne la forme (shape) de la matrice RustArray
            &self.shape
        }
        pub fn get_kind(&self) -> &MatrixKind {
            //Retourne le type (kind) de la matrice RustArray
            &self.kind
        }
    }
    pub fn test(){
        print!("test");
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

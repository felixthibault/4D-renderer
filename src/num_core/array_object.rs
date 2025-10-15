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
        view: Option<String>, /*On peut voir une matrice verticalement(C, valeur par défaut) ou horizontalement(F).
                        On peut définir la vue en colonne (C) et une vue Fortran selon la façon que l'on perçoit une matrice.
            La matrice  [a  b  c|
                        |d  e  f|
                        |g  h  i] par l'array de tuple [(a,b,c), (d,e,f), (g,h,i)]. Selon ce que l'on préfère, on peut compter
            le nombre d'éléments dans chaque tuple et ensuite compter le nombre de tuples et afficher cela dans un nouveau nombre:
            (n,m). n et m ne signifient donc pas la longueur et la hauteur de façon absolue. Dans la vue Fortran, n devient la hauteur
            alors que dans la vue en colonnes, n est la longueur. Dans tous les cas, n est la longueur d'un tuple et m est la longueur
            de la liste de tuples, même si on ne sait jamais vraiment qu'est ce qui est m ou n sans regarder si view est "C" ou "F".
            Dans la vue en colonne (C), la visualisation affichée est (n,m) ou (longueur, hauteur).
            Dans la vue Fortran (F), la visualisation affichée est (m,n) ou (hauteur, longueur).
            */
        buffer: Option<buffer>, //Un nombre connu de bytes peut être inséré en valeur de l'array.
        offset: usize, //Il peut y avoir un décalage de x octets dans la lecture du buffer (Offset of array data in buffer).
                       //Si une structure est présente dans la mémoire, ses bytes seront sautées de l'array.
        strides: Option<(usize,usize)>, //Le décalage d'octets dans le buffer peut être spécifié pour une quantité dans chaque dimension.
                                        //Permet un contrôle encore plus fin de la mémoire que strides. Utile pour les vues non-contiguës, transposées.
                                    }

    impl<H> RustArray<H>{
        pub const fn new() -> RustArray<H> {
            //Crée une nouvelle matrice vierge, dynamique et sous forme de matrice de transformation.
            RustArray{
                data: Vec::new(),
                shape: None,
                kind: MatrixKind::Trans,
                view: None,
                buffer: None,
                offset: 0usize,
                strides: None,
            }
        }
        pub fn with_capacity(capacite:usize) -> RustArray<H> {
            //Crée une nouvelle matrice de transformation vierge, mais avec une capacité prédéfinie si elle est connue
            RustArray{
                data: Vec::with_capacity(capacite),
                shape: None,
                kind: MatrixKind::Trans,
                view: None,
                buffer: None,
                offset: 0usize,
                strides: None,
            }
        }
        pub fn zeros(shapes:(usize, usize)) -> RustArray<H> where H: num::Zero + std::clone::Clone + Copy {
            //Crée un array de 0 de type H de forme shapes
            let longueur=shapes.0*shapes.1;
            RustArray{
                data: vec![H::zero();longueur],
                shape: Some(shapes),
                kind: MatrixKind::Trans,
                view: None,
                buffer: None,
                offset: 0usize,
                strides: None,
            }
        }
        pub fn reshape(&mut self, shape:(usize, usize)) -> Option<i8> {
            /*Reforme une matrice unidimensionnelle à une forme multidimensionnelle.
            Si la matrice a déjà une forme=!(x,0), la fonction va vérifier ce qui est possible de modifier.
            Nécessite un tuple de 2 arguments.
            Pour des cas particuliers où une dimension est inconnue, envoyer à , unknown_reshape.
            self: vecteur-style à être reformé
            shape: tuple(usize ou u32)
            */
            let x=self.data;
            if x.len() != shape.0*shape.1 {
                //Forme est nulle ou longueur du vecteur inadaptée pour le volume de la matrice
                None
            }
            else{
                self.shape=Some(shape);
                return 1
            }
        }
        pub fn unknown_reshape(&mut self, shape:(isize,isize)) ->Option<i8>{
            /*Reforme une matrice unidimensionnelle ou avec une forme en une matrice multidimensionnelle.
            Puisque un des nombres du tuple est supposé être -1 (donc que sa mesure est inconnue), deux inconnus sera refusé.
            Nécessite un tuple de 2 arguments. Si deux nombres>1=>reshape()
            Le programme va essayer de voir quelles combinaisons sont possibles.
            self: vecteur-style à être reformé
            shape: tuple(isize ou i32)
            */
            let x=&self.data;
            if shape.0 >=0 && shape.1 >=0 {(&mut self).reshape(Some(shape.0 as usize, shape.1 as usize));}
            else if shape.0 ==-1 && shape.1 >=0 {
                //Reshape et chercher longueur
                if shape!=self.shape.unwrap() && (x.len() as f32/shape.1 as f32-(x.len()/shape.1) as f32)==0.{
                    self.shape=Some( (x.len()/shape.1,shape.1) );
                }
                return None
            } 
            else if shape.0 >=0 && shape.1 ==-1 {
                //Reshape et chercher hauteur
                if shape!=self.shape.unwrap() && (x.len() as f32/shape.0 as f32-(x.len()/shape.0) as f32)==0.{
                    self.shape=Some( (shape.0,x.len()/shape.0) );
                }
                return Some(1)
            }
            else {
                //aucune combinaison gagnante (mauvais envoi ou deux inconnus)
                return None
            }
        }
        pub fn flatten(&mut self){ 
            //Rend un objet RustArray multidimensionnel à une forme 1D aplatie
            self.shape=None;//ou self.shape=Some((self.data.len(),1));
        }
        pub fn push_col(&mut self, col: Vec<H>)-> Option<i8> {
            //Pousse une colonne dans un objet RustArray. Retourne une erreur si la colonne ne correspond pas à la hauteur de la matrice.
            let mut shape_unwraped=&self.shape.unwrap();
            let (column,position_array)=match self.view{
                //column: Vérification selon le type de visualisation l'élément colonne du tuple de shape
                //position_array: Ajouter la colonne selon la forme de self et la vue (Fortran ou par colonne)
                "C".to_string()=>(shape_unwraped.1,shape_unwraped.0),
                "F".to_string()=>(shape_unwraped.0,shape_unwraped.1),
                None=>(shape_unwraped.1,shape_unwraped.0),
            };

            if col.len()!=column{
                //Colonne de longueur incorrecte
                None
            }
            
            for money in 0..col.len(){
                self.data.insert(position_array*(money+1)+money,col[money]);
            }
            match self.view{
                //Mofidier shape_unwraped selon l'ajout
                "C".to_string()=>shape_unwraped=(position_array+1,shape_unwraped.1),
                "F".to_string()=>shape_unwraped=(shape_unwraped.0,position_array+1),
                None=>{shape_unwraped=(position_array+1,shape_unwraped.1);self.view="C".to_string()},
            }
            assert_eq(self.data.len(),shape_unwraped.0*shape_unwraped.1);
            None
        }
        pub fn push_row(&mut self, row: Vec<H>)-> Option<i8> {
            //Pousse une ligne dans un objet RustArray. Retourne une erreur si la ligne ne correspond pas à la longueur de la matrice.
            let mut shape_unwraped=&self.shape.unwrap();
            let (rangee,position_array)=match self.view{
                //rangee: Vérification selon le type de visualisation l'élément rangée du tuple de shape
                //position_array: Ajouter la ligne selon la forme de self et la vue (Fortran ou par colonne)
                "C".to_string()=>(shape_unwraped.0,shape_unwraped.1),
                "F".to_string()=>(shape_unwraped.1,shape_unwraped.0),
                None=>(shape_unwraped.0,shape_unwraped.1),
            };

            if row.len()!=rangee{
                //Rangée de longueur incorrecte
                None
            }
            
            for money in 0..row.len(){
                self.data.insert(position_array*(money+1)+money,row[money]);
            }
            match self.view{
                //Mofidier shape_unwraped selon l'ajout
                "C".to_string()=>shape_unwraped=(shape_unwraped.0,position_array+1),
                "F".to_string()=>shape_unwraped=(position_array+1,shape_unwraped.1),
                None=>{shape_unwraped=(shape_unwraped.0,position_array+1);self.view="C".to_string()},
            }
            assert_eq(self.data.len(),shape_unwraped.0*shape_unwraped.1);
            None
        }
        pub fn swap(&mut self, a:isize, b:isize) where H: Copy{
            //Swap deux éléments dans un array depuis les index a et b
            let mut data=&self.data;
            let len =data.len() as isize;//Attribution des références au valeurs
            
            let c= if a<0 {len+a} else {a}; 
            let d= if b<0 {len+b} else {b};//Si a ou b est négatif, on reréfère à l'index à la fin de la liste
            
            // Vérifie que les indices sont valides
            assert!(c >= 0 && c < len, "Index a invalide");
            assert!(d >= 0 && d < len, "Index b invalide");
            let c=c as usize;
            let d=d as usize;
            let temp=data[c];//Attribution de la valeur de a temporairement
            
            //Change les valeurs
            data[c]=data[d];//Déplacement de b dans a
            data[d]=temp;//Déplacement de a dans b
        }
        pub unsafe fn unsafe_swap(&mut self, a:usize, b:usize){
            assert_unsafe_precondition!(
                check_library_ub,
                "slice::swap_unchecked requires that the indices are within the slice",
                (
                    len: usize = self.len(),
                    a: usize = a,
                    b: usize = b,
                ) => a < len && b < len,
            );
            let x=self.data;
            unsafe{
                x[a]=x[a]^x[b];
                x[b]=x[b]^x[a];
                x[a]=x[a]^x[b];//si tout fonctionne, devrait fonctionner
            }

        }
        pub fn transpose(&mut self){
            ///Take a square matrix and transpose it so that its columns become
            ///his rows.
            /*
            //Vérifier si forme existe
            assert!(self.shape=!None);
            let data=&self.data;
            //Transposer sur une nouvelle matrice
            let row=self.shape.0;
            let col=self.shape.1;
            let mut data_temp={
                let x:Vec<H>=Vec::new(); 
                for i in 0..row{
                    for j in i+1..row{
                        x.push(data[j*col]);//Vérifier le calcul
                    }
                }
                x
            }
            let temp=RustArray{data:data_temp, shape:(self.shape.1,self.shape.0),kind:self.kind};
            ///Idée de correction, changer la vision de la matrice à la place des valeurs.
            */

            //Correction: changer la valeur de view
            if self.view=="F".to_string(){
                self.view="C".to_string(); //Set the alue in Column view style
            }
            else{
                self.view="F".to_string(); //Set the view in a Fortran style
            }
            //Ici shape n'est pas modifiée puisque la forme n'est pas absolue et dépend de la visualisation.
            //Si une matrice est de longueur 3(n) et de hauteur 5(m), sa transpose est de longueur 5(m) et 
            //de hauteur 3(n). La forme serait cependant toujours Some((3,5)).
        }
        
        pub fn get_data(&self) -> &Vec<H>{
            //Retourne les données (data) contenues dans la matrice RustArray 
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
        pub fn get_view(&self) -> &Option<String>{
            //Retourne la visualisation (view) de la matrice RustArray
            &self.view
        }
        pub fn get_buffer(&self) -> &Option<buffer>{
            //Retourne le stockage prérempli (buffer) de la matrice RustArray
            &self.buffer
        }
        pub fn get_offset(&self) -> &usize{
            //Retourne le décalage (offset) dans la lecture du buffer de la matrice RustArray
            &self.offset
        }
        pub fn get_strides(&self) -> &Option<(usize,usize)>{
            //Retourne le recul horizontal et vertical (strides) du buffer de la matrice RustArray
            &self.strides
        }
        pub fn as_arr_mut_ptr(&mut self) -> *mut T {//Même chose que as_mut_ptr, mais pour une matrice
            //Take self (&mut self) as a mutable reference and returns a mutable raw pointer to
            //the first element in the internal slice of T.
            self.data as *mut [T] as *mut T
        }
        
        //Opérations mathématiques sur les matrices de base non spécifiées. 
        //Cette section comporte: l'addition, le produit de deux matrices, le produit d'une 
        //série de matrices, la multiplication par un scalaire.
        pub fn scalable_matrix
        pub fn add_matrix
        pub fn produit_matrices
        pub fn produit_multi_matrices

        //Opérations de Gauss et Élémentaires Lignes pour les matrices équations.
        //Cette section comporte: les opérations élémentaires ligne (OEL) la résolution complète
        //de matrices, le nombre de pivots, le nombre d'équations, le nombre de variables, 
        //l'échelonnage de matrice de Gauss (SEL), l'échelonnage de matrice de Gauss-Jordan (SELH).


        //Opérations systèmes pour les matrices de structures comme des objets multidimensionnels.

    }
    pub fn test(){
        print!("test");
    }
    
//se référer à numpy.ndarray pour trouver des idées de fonctions: https://numpy.org/doc/1.22/reference/generated/numpy.ndarray.html?
// ainsi qu'à vec.Vec: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
//inspiration: https://github.com/IgorSusmelj/rustynum/blob/main/docs/tutorials/better-matrix-operations.md
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

pub fn from_elem<H: Clone>(elem: H, n: usize) -> RustArray<H> {
    <H as SpecFromElem>::from_elem(elem, n, Global)
}

pub const fn as_mut_ptr(&mut self) -> *mut T {//Aucune idée de ce truc
    //Take self (&mut self) as a mutable reference and returns a mutable raw pointer (ptr) to
    //the first element in the internal slice of T.
    self as *mut [T] as *mut T
}
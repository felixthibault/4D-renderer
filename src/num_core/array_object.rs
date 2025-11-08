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
use std::{fmt::{self, Display, Debug,}, mem,};

pub mod(super) Array{
    enum MatrixKind{
        Eq, //Equation Matrix type, système d'équations linéaires
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
        kind: MatrixKind,// La matrice peut représenter une transformation linéaire, un système d'équations linéaires ou des objets.
        view: Option<String>, /*On peut voir une matrice verticalement(C, valeur par défaut) ou horizontalement(F).
                        On peut définir la vue en colonne (C) et une vue Fortran selon la façon que l'on perçoit une matrice.
            La matrice  [a  b  c|
                        |d  e  f|
                        |g  h  i] peut être lue comme un array de tuples [(a,b,c), (d,e,f), (g,h,i)]. Selon ce que l'on préfère, on 
            peut compter le nombre d'éléments dans chaque tuple et ensuite compter le nombre de tuples et afficher cela dans un nouveau
            nombre: (n,m) ou (i,j). n et m signifient respectivement le nombre de lignes et le nombre de colonnes (de façon absolue).
            Dans la vue Fortran, on lit les éléments de haut en bas et ensuite de gauche à droite, alors que dans la vue en colonnes,
            on lit de gauche à droite et ensuite de haut en bas. Selon le cas, n et m peuvent être le nombre de tuple dans la liste ou
            le nombre d'éléments dans un tuple. Selon la visualisation, l'ordre des données n'est pas la même, donc que la lecture en
            tuple va aussi changer. Pour l'exemple plus haut, il est aussi possible de lire la matrice comme l'array 
            [(a,d,g), (b,e,h), (c,f,i)], pour une vue de style Fortran. La forme est cependant indépendante de view= "C" ou "F".
            Dans la vue en colonne (C), la visualisation affichée est horizontale.
            Dans la vue Fortran (F), la visualisation affichée est verticale.
            */
        buffer: Option<buffer>, //Un nombre connu de bytes peut être inséré en valeur de l'array.
        offset: usize, //Il peut y avoir un décalage de x octets dans la lecture du buffer (Offset of array data in buffer).
                       //Si une structure est présente dans la mémoire, ses bytes seront sautées de l'array.
        strides: Option<(usize,usize)>, //Le décalage d'octets dans le buffer peut être spécifié pour une quantité dans chaque dimension.
                                        //Permet un contrôle encore plus fin de la mémoire que strides. Utile pour les vues non-contiguës, transposées.
        }

    impl<H,T> RustArray<H,T>{
        //Create new arrays
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
        pub const fn array(data:[H]) -> RustArray<H> {
            //Crée une nouvelle matrice de transformation avec les données contenues dans data.
            //Note: Make sure that data.len() is not a prime number before to send it. 
            //Otherwise, it's impossible to reshape it.
            RustArray{
                data: data.to_vec(),
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
        pub fn copy(&self)-> self {
            self//À vérifier comment créer une copie d'une instance d'array
        }

        //Change formatting
        pub fn reshape(&mut self, shape:(usize, usize)) -> Option<i8> {
            /*Reforme une matrice unidimensionnelle à une forme multidimensionnelle.
            Si la matrice a déjà une forme=!(x,0), la fonction va vérifier ce qui est possible de modifier.
            Nécessite un tuple de 2 arguments. *Faire attention que data.len() n'égale pas un nombre premier.
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
                return Some(1)
            }
        }
        pub fn unknown_reshape(&mut self, shape:(isize,isize)) ->Option<i8>{
            /*Reforme une matrice unidimensionnelle ou avec une forme en une matrice multidimensionnelle.
            Puisque un des nombres du tuple est supposé être -1 (donc que sa mesure est inconnue), deux inconnus sera refusé.
            Nécessite un tuple de 2 arguments. Si deux nombres>1 =>reshape()
            Le programme va essayer de voir quelles combinaisons sont possibles.
            self: vecteur-style à être reformé
            shape: tuple(isize)
            */
            let x=&self.data;
            if shape.0 >=0 && shape.1 >=0 {(&mut self).reshape(shape.0 as usize, shape.1 as usize)}
            else if shape.0 ==-1 && shape.1 >=0 {
                //Reshape et chercher hauteur
                //Il faut vérifier que la forme est divisable en entier et que self.shape!=None
                if self.shape!=None{
                    if shape.1==self.shape.unwrap().1{None}
                }
                let shapes=&shape.1 as usize;
                if calcul(shapes) {
                    //Si ça passe, on peut changer self.shape
                    self.shape=Some( (x.len()/shapes,shapes) );
                    return Some(1)
                }
                return None
            } 
            else if shape.0 >=0 && shape.1 ==-1 {
                //Reshape et chercher longueur
                //Il faut vérifier que la forme est divisable en entier et que self.shape!=None
                if self.shape!=None{
                    if shape.0==self.shape.unwrap().0{None}
                }
                let shapes=&shape.0 as usize;
                if calcul(shapes) {
                    //Si ça passe, on peut changer self.shape
                    self.shape=Some( (shapes,x.len()/shapes) );
                    return Some(1)
                }
                return None
            }
            else {
                //aucune combinaison gagnante (mauvais envoi ou deux inconnus)
                return None
            }
            fn calcul (&shape:usize) -> bool {
                //if shape!=self.shape.unwrap() && (x.len() as f32/shape.1 as f32-(x.len()/shape.1) as f32)==0.
                return x.len() % shape==0
                //On retourne faux si la longueur de data n'est pas divisable par shape
            }
        }
        pub fn flatten(&mut self){ 
            //Rend un objet RustArray multidimensionnel à une forme 1D aplatie
            self.shape=None;//ou self.shape=Some((1,self.data.len()));
        }
        pub fn push_col(&mut self, col: Vec<H>)-> Option<i8> {
            ///Pousse une colonne dans un objet RustArray. 
            ///Retourne une erreur si la colonne ne correspond pas à la hauteur de la matrice.
            /// Panique si shape=None
            let mut shape_unwraped=&self.shape.unwrap();
            let (column,position_array)=
                //column: Longueur de l'élément colonne du tuple de shape
                //position_array: Ajouter la colonne selon la forme de self (correspond à j)
                (&shape_unwraped.0,&shape_unwraped.1);

            if col.len()!=column{
                //Colonne de longueur incorrecte
                Some(-1)
            }
            
            
            match self.view{
                //Mofidier shape_unwraped selon la visualisation et l'ajout
                "C".to_string()=>{
                    for money in 0..col.len(){
                        self.data.insert(position_array*(money+1)+money+1,col[money]);
                        //Fonctionne en "C", mais pas en "F"
                    };
                    },
                "F".to_string()=>{
                    for money in col{
                        //Insérer à la fin de data
                        self.data.push(money);
                    };
                    },
                None=>{
                    for money in 0..col.len(){
                        self.data.insert(position_array*(money+1)+money+1,col[money]);
                        //Fonctionne en "C", mais pas en "F"
                    };
                    self.view="C".to_string();
                    },
            }
            shape_unwraped=(&shape_unwraped.0,&shape_unwraped.1+1);

            assert_eq(self.data.len(),shape_unwraped.0*shape_unwraped.1);
            self.shape=Some(shape_unwraped);
            None
        }
        pub fn push_row(&mut self, row: Vec<H>)-> Option<i8> {
            ///Pousse une ligne dans un objet RustArray. 
            ///Retourne une erreur si la ligne ne correspond pas à la longueur de la matrice.
            /// Panique si shape==None
            let mut shape_unwraped=&self.shape.unwrap();
            let (rangee,position_array)=
                //rangee: Longueur de l'élément ligne du tuple de shape
                //position_array: Ajouter la ligne selon la forme de self (correspond à i)
                (&shape_unwraped.1,&shape_unwraped.0);

            if row.len()!=rangee{
                //Ligne de longueur incorrecte
                Some(-1)
            }
            
            
            match self.view{
                //Mofidier shape_unwraped selon la visualisation et l'ajout
                "C".to_string()=>{
                    for money in row{
                        //Insérer à la fin de data
                        self.data.push(money);
                        //Fonctionne en "C", mais pas en "F"
                    };
                    },
                "F".to_string()=>{
                    for money in 0..row.len(){
                        self.data.insert(position_array*(money+1)+money+1,row[money]);
                    };
                    },
                None=>{
                    for money in row{
                        //Insérer à la fin de data
                        self.data.push(money);
                        //Fonctionne en "C", mais pas en "F"
                    };
                    self.view="C".to_string();
                    },
            }
            shape_unwraped=(&shape_unwraped.0+1,&shape_unwraped.1);

            assert_eq(self.data.len(),shape_unwraped.0*shape_unwraped.1);
            self.shape=Some(shape_unwraped);
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

            //Correction: changer la valeur de view et de shape
            if self.view=="F".to_string(){
                self.view="C".to_string(); //Set the view in Column view style
            }
            else{
                self.view="F".to_string(); //Set the view in a Fortran style
            }
            ///Ici shape est modifiée puisque la forme est absolue et ne dépend pas de la visualisation.
            ///ij -> ji
            ///Si une matrice est de longueur 3(m) et de hauteur 5(n), sa transpose est de longueur 5(m) et 
            ///de hauteur 3(n). La forme serait aussi Some((5,3)).
            if self.shape==None{
                self.shape=Some((self.data.len(),1));
            }
            else{
                let shape_unwraped=&self.shape.unwrap()
                let mut (a,b)=(&shape_unwraped.0,&shape_unwraped.1);
                mem::swap(&mut a, &mut b)
                self.shape=Some( (a,b));
            }
        }
        pub fn T(&mut self){
            //Autre façon d'écrire transpose d'une matrice
            /*let x=RustArray::new */
            transpose(&mut self);
        }
        pub fn to_fortran(&mut self){
            ///Convertie une visualisation par colonne en fortran.
            ///On crée une nouvelle matrice contenant un vecteur de référence aux positions 
            /// de l'ancien vecteur. Formule pour trouver n2=(n-1)%j*i+(n-1)//j+1
            ///Vérifier si shape==None
            if self.shape==None{
                self.reshape((self.data.len(),1));
                self //Séquence terminée
            }
            let shape=&self.shape.unwrap();
            let (i,j)=(shape.0,shape.1);
            self.data=self.data.into_iter() .map(|n|{(n-1)%j*i+(n-1)/j+1}) .collect();
            //On garde la même forme.
        }
        pub fn to_column(&mut self){
            ///Même chose que de passer de colonne à fortran en inversant i et j.
            ///Convertie une visualisation fortran en colonne.
            ///On crée une nouvelle matrice contenant un vecteur de référence aux positions 
            /// de l'ancien vecteur. Formule pour trouver n2=(n-1)%i*j+(n-1)//i+1
            if self.shape==None{
                self.reshape((1,self.data.len()));
                self //Séquence terminée
            }
            let shape=&self.shape.unwrap();
            let (i,j)=(shape.0,shape.1);
            self.data=self.data.into_iter() .map(|n|{(n-1)%i*j+(n-1)/i+1}) .collect();
            //On garde la même forme.
        }
        
        //Get the values
        pub fn get_data(&self) -> &Vec<H>{
            //Retourne les données (data) contenues dans la matrice RustArray 
            //*Pour la lecture d'une variable, si x=array{data=[...]}; data= x.data
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
        
        //Get pointer to elements in data
        pub fn as_arr_mut_ptr(&mut self) -> *mut T {//Même chose que as_mut_ptr, mais pour une matrice
            //Take self (&mut self) as a mutable reference and returns a mutable raw pointer to
            //the first element in the internal slice of T.
            self.data as *mut [T] as *mut T
        }
        
    }
    
    impl<H: Display> Display for RustArray<H> {
        pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //Imprime les valeurs contenues dans la matrice selon le formatage de shape 
            //et la visualisation Fortran ou par colonne
            ///Permet de faire print!("{}", x); ou println!("{}", x);
            let shaped:Vec<Vec<H>>= match self.view {
                "F".to_string()=> Fortran_into_vec(&self)
                "C".to_string()=> Column_into_vec(&self)
            }
            write!(f, "Donnée: {}", shaped)
        }
        pub fn fortran_into_vec(&self) -> Vec<Vec<H>{
            let big_vec:Vec<Vec<H>>=Vec::new();
            let forme=self.shape.unwrap();
            for &donnee in self.data{
                let mut n:usize=0
                let mut vec=Vec::new();
                while n<forme.1{
                    //Si nombre de ligne n'est pas atteint, on continu
                    vec.add(donnee);
                    n+=1
                }
                big_vec.add(vec);
            }
            big_vec
        }
        pub fn column_into_vec(&self) -> Vec<Vec<H>{
            let big_vec:Vec<Vec<H>>=Vec::new();
            let forme=self.shape.unwrap();
            for &donnee in self.data{
                let mut n:usize=0
                let mut vec=Vec::new();
                while n<forme.0{
                    //Si nombre de ligne n'est pas atteint, on continu
                    vec.add(donnee);
                    n+=1
                }
                big_vec.add(vec);
            }
            big_vec
        }
    }

    impl<H: Debug> Debug for RustArray<H> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //Imprime les valeurs contenues dans data sans regarder la forme
            //Permet de faire dbg!(x);
            f.debug_struct("RustArray")
                .field("data", &self.data)
                .finish()
        }
    }
    
    pub fn afficher(&self)
            where H: Display, {
            //Imprime les valeurs stockées dans la matrice
            print!("Données: {}",self.data);
        }
    pub fn sort(&mut self)
        where H:Ord,{
            //Trie les valeurs dans l'ordre alphabétique à l'intérieur d'une nouvelle array.
            ///Exemple d'utilisation: 
            /*
                let mut v = Rarray::array([4, -5, 1, -3, 2]);
                v.sort();
                assert_eq!(v.data, [-5, -3, 1, 2, 4].to_vec());
            */
            self.data.sort();
        }
    pub fn sort_unstable(&mut self)
        where H:Ord,{
            //Trie les valeurs dans l'ordre alphabétique à l'intérieur d'une array.
            //Fonction typiquement plus rapide que sort, mais peut paniquer.
            //Dans le cas des f32 et f64, où f32::Nan!=f32::Nan, il faut utiliser cette fonction.
            ///Exemple d'utilisation: 
            /*
                let mut v = Rarray::array([4, -5, 1, -3, 2]);
                v.sort_unstable();
                assert_eq!(v.data, [-5, -3, 1, 2, 4].to_vec());
            */
            self.data.sort_unstable();
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
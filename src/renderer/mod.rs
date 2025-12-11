pub mod objets;
pub mod transformations;
pub mod embarquation_b4d;
pub mod winsdl;
//mod macros;


pub fn test(){
    //Appel de tous les test fonctionnels
    objets::test();
    transformations::test();
    embarquation_b4d::test();
    winsdl::test();
}
#[derive(Plugins)]
pub struct RendererPlugin;

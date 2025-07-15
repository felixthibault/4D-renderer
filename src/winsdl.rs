use bevy::prelude::*;

fn main() {
    App::new()
    .world()
    .run();
}
//-----Components--------
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
//-----Systems--------
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
fn hello_world() {
    println!("hello world!");
}
//-----Entities--------
struct Entity(u64);
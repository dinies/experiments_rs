extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::scene::{SceneNode, PlanarSceneNode};
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector2, Vector3, UnitComplex, Translation2, Isometry2};
use rand::Rng;
use rand::seq::SliceRandom;

struct Tile{
    node: PlanarSceneNode,
    // position: Translation2<f32>,
    // colour: Vector3<f32>
}
impl Tile{
    pub fn new(
        node: PlanarSceneNode,
        position: Translation2<f32>,
        colour: Vector3<f32>
    ) -> Self {
        Self {
            node,
            // position,
            // colour,
        }
    }
}


struct CheckerBoard {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols:usize,
    color_empty: Vector3<f32>,
    color_full: Vector3<f32>,
}
impl CheckerBoard{
pub fn new(window: &mut kiss3d::window::Window, rows: usize, cols:usize, width_tile: f32, height_tile:f32, color_empty: Vector3<f32>, color_full: Vector3<f32>) -> Self {

        let mut matrix = Vec::with_capacity(rows);
        for r in 0..rows{
            let mut column = Vec::with_capacity(cols);
            for c in 0..cols{
                let mut node = window.add_rectangle(width_tile, height_tile);
                node.set_color(color_empty[0], color_empty[1],color_empty[2]);
                let position = Translation2::from(Vector2::new(c as f32 *width_tile + width_tile/2.0, r as f32 *height_tile+ height_tile/2.0));
                node.append_translation(&position);

                let tile: Tile = Tile::new(node, position, color_empty);
                column.push(tile);
            }
            matrix.push(column);
        }


        Self {
            tiles: matrix,
            rows,
            cols,
            color_empty,
            color_full
        }
    }
}
impl State for CheckerBoard{
    fn step(&mut self, _: &mut Window) {
        let mut rng = rand::thread_rng();
        let random_row = rng.gen_range(0..self.rows);
        let random_col = rng.gen_range(0..self.cols);
        let random_float = rng.gen_range(0.0..=1.0);
        if random_float > 0.5 {
            let color: Vector3<f32> = self.color_full;
            self.tiles[random_row][random_col].node.set_color(color[0],color[1],color[2])
        }
        else{
             let color: Vector3<f32> = self.color_empty;
            self.tiles[random_row][random_col].node.set_color(color[0],color[1],color[2])
        };
    }

}



struct AppState {
    c: PlanarSceneNode,
    rot: UnitComplex<f32>,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: wasm example");
    // let mut c = window.add_rectangle(30.0, 20.0);

    // c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    // let rot = UnitComplex::new(0.01);
    // let state = AppState { c, rot };

    let state : CheckerBoard = CheckerBoard::new(&mut window, 20, 20, 50.0, 50.0, Vector3::<f32>::new(1.0,0.0,0.0),Vector3::<f32>::new(0.0,0.0,1.0) ) ;

    window.render_loop(state)
}

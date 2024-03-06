extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation2, Vector2, Vector3};
use kiss3d::scene::PlanarSceneNode;
use kiss3d::window::{State, Window};
use rand::Rng;

struct Tile {
    node: PlanarSceneNode,
}
impl Tile {
    pub fn new(node: PlanarSceneNode) -> Self {
        Self { node }
    }
}

struct CheckerBoard {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
    color_empty: Vector3<f32>,
    color_full: Vector3<f32>,
}
impl CheckerBoard {
    pub fn new(
        window: &mut kiss3d::window::Window,
        rows: usize,
        cols: usize,
        width_tile: f32,
        height_tile: f32,
        color_empty: Vector3<f32>,
        color_full: Vector3<f32>,
    ) -> Self {
        let mut matrix = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut column = Vec::with_capacity(cols);
            for c in 0..cols {
                let mut node = window.add_rectangle(width_tile, height_tile);
                node.set_color(color_empty[0], color_empty[1], color_empty[2]);
                let position = Translation2::from(Vector2::new(
                    c as f32 * width_tile + width_tile / 2.0 - cols as f32 * width_tile / 2.0,
                    r as f32 * height_tile + height_tile / 2.0 - rows as f32 * height_tile / 2.0,
                ));
                node.append_translation(&position);

                let tile: Tile = Tile::new(node);
                column.push(tile);
            }
            matrix.push(column);
        }

        Self {
            tiles: matrix,
            rows,
            cols,
            color_empty,
            color_full,
        }
    }
}
impl State for CheckerBoard {
    fn step(&mut self, _: &mut Window) {
        let mut rng = rand::thread_rng();
        let random_row = rng.gen_range(0..self.rows);
        let random_col = rng.gen_range(0..self.cols);
        let random_float = rng.gen_range(0.0..=1.0);
        if random_float > 0.5 {
            let color: Vector3<f32> = self.color_full;
            self.tiles[random_row][random_col]
                .node
                .set_color(color[0], color[1], color[2])
        } else {
            let color: Vector3<f32> = self.color_empty;
            self.tiles[random_row][random_col]
                .node
                .set_color(color[0], color[1], color[2])
        };
    }
}

fn main() {
    let mut window = Window::new("Tiles board");

    window.set_light(Light::StickToCamera);
    let state: CheckerBoard = CheckerBoard::new(
        &mut window,
        20,
        20,
        50.0,
        50.0,
        Vector3::<f32>::new(226.0 / 255.0, 135.0 / 255.0, 67.0 / 255.0),
        Vector3::<f32>::new(6.0 / 255.0, 57.0 / 255.0, 112.0 / 255.0),
    );

    window.render_loop(state)
}

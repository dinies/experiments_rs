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

#[derive(Clone, Copy, Debug)]
struct Coords2D {
    row: usize,
    col: usize,
}

impl Coords2D {
    pub fn new(r: usize, c: usize) -> Self {
        Self { row: r, col: c }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum CellState {
    Alive,
    Dead,
}

struct Board {
    rows: usize,
    cols: usize,
    grid_state: Vec<Vec<CellState>>,
    grid_graphics: Vec<Vec<Tile>>,
    color_dead: Vector3<f32>,
    color_alive: Vector3<f32>,
}

impl Board {
    pub fn new(
        window: &mut kiss3d::window::Window,
        rows: usize,
        cols: usize,
        width_tile: f32,
        height_tile: f32,
        color_dead: Vector3<f32>,
        color_alive: Vector3<f32>,
    ) -> Self {
        let mut matrix_graphics = Vec::with_capacity(rows);
        let mut matrix_state = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut column_state = Vec::with_capacity(cols);
            let mut column_graphics = Vec::with_capacity(cols);
            for c in 0..cols {
                let mut node = window.add_rectangle(width_tile, height_tile);
                node.set_color(color_dead[0], color_dead[1], color_dead[2]);
                let position = Translation2::from(Vector2::new(
                    c as f32 * width_tile + width_tile / 2.0 - cols as f32 * width_tile / 2.0,
                    r as f32 * height_tile + height_tile / 2.0 - rows as f32 * height_tile / 2.0,
                ));
                node.append_translation(&position);

                let tile: Tile = Tile::new(node);
                column_state.push(CellState::Dead);
                column_graphics.push(tile);
            }
            matrix_state.push(column_state);
            matrix_graphics.push(column_graphics);
        }

        Self {
            grid_graphics: matrix_graphics,
            grid_state: matrix_state,
            rows,
            cols,
            color_dead,
            color_alive,
        }
    }

    pub fn is_alive(grid_state: &[Vec<CellState>], coords: Coords2D) -> bool {
        grid_state[coords.row][coords.col] == CellState::Alive
    }

    pub fn set_alive(&mut self, coords: Coords2D) {
        self.grid_graphics[coords.row][coords.col].node.set_color(
            self.color_alive[0],
            self.color_alive[1],
            self.color_alive[2],
        );
        self.grid_state[coords.row][coords.col] = CellState::Alive;
    }

    #[allow(dead_code)]
    pub fn set_alive_in_place(&self, tile: &mut Tile, state: &mut CellState) {
        tile.node.set_color(
            self.color_alive[0],
            self.color_alive[1],
            self.color_alive[2],
        );
        *state = CellState::Alive;
    }

    pub fn set_dead(&mut self, coords: Coords2D) {
        self.grid_graphics[coords.row][coords.col].node.set_color(
            self.color_dead[0],
            self.color_dead[1],
            self.color_dead[2],
        );
        self.grid_state[coords.row][coords.col] = CellState::Dead;
    }

    #[allow(dead_code)]
    pub fn set_dead_in_place(&self, tile: &mut Tile, state: &mut CellState) {
        tile.node
            .set_color(self.color_dead[0], self.color_dead[1], self.color_dead[2]);
        *state = CellState::Dead;
    }

    fn is_valid_square(&self, row: usize, col: usize) -> bool {
        !(row >= self.rows || col >= self.cols)
    }

    fn get_neighbors(&self, coord: Coords2D) -> Vec<Coords2D> {
        let mut neighbors: Vec<Coords2D> = Vec::with_capacity(8);
        let mut neighbors_coords: Vec<(usize, usize)> = Vec::with_capacity(8);
        let rows_upper_bound: usize = self.rows - 1;
        let cols_upper_bound: usize = self.cols - 1;
        if coord.row > 0 {
            neighbors_coords.push((coord.row - 1, coord.col));
            if coord.col > 0 {
                neighbors_coords.push((coord.row - 1, coord.col - 1));
            }
        }
        if coord.row < rows_upper_bound {
            neighbors_coords.push((coord.row + 1, coord.col));
            if coord.col < cols_upper_bound {
                neighbors_coords.push((coord.row + 1, coord.col + 1));
            }
        }
        if coord.col > 0 {
            neighbors_coords.push((coord.row, coord.col - 1));
            if coord.row < rows_upper_bound {
                neighbors_coords.push((coord.row + 1, coord.col - 1));
            }
        }
        if coord.col < cols_upper_bound {
            neighbors_coords.push((coord.row, coord.col + 1));
            if coord.row > 0 {
                neighbors_coords.push((coord.row - 1, coord.col + 1));
            }
        }

        for (coord_row, coord_col) in neighbors_coords {
            if self.is_valid_square(coord_row, coord_col) {
                neighbors.push(Coords2D::new(coord_row, coord_col));
            }
        }
        neighbors
    }
}

struct Simulation {
    world: Box<Board>,
}

impl Simulation {
    pub fn new(mut board: Box<Board>, initial_configuration: Vec<Coords2D>) -> Self {
        for coord in initial_configuration {
            board.set_alive(coord);
        }
        Self { world: board }
    }
    pub fn count_alive_neighbors(
        neighbors: &Vec<Coords2D>,
        grid_state: &[Vec<CellState>],
    ) -> usize {
        let mut num_of_alive_neighbors: usize = 0;
        for neighbor in neighbors {
            let is_neighbor_alive: bool = Board::is_alive(grid_state, *neighbor);
            if is_neighbor_alive {
                num_of_alive_neighbors += 1;
            }
        }
        num_of_alive_neighbors
    }

    pub fn apply_conway_rule(
        &mut self,
        curr_coord: &Coords2D,
        num_of_alive_neighbors: usize,
        grid_state: &[Vec<CellState>],
    ) {
        let is_alive = Board::is_alive(grid_state, *curr_coord);
        if is_alive {
            if !(2..=3).contains(&num_of_alive_neighbors) {
                self.world.set_dead(*curr_coord);
            }
        } else if num_of_alive_neighbors == 3 {
            self.world.set_alive(*curr_coord);
        }
    }

    pub fn evolve(&mut self) {
        let grid_state: Vec<Vec<CellState>> = self.world.grid_state.clone();
        for r in 0..self.world.rows {
            for c in 0..self.world.cols {
                let curr_coord = Coords2D::new(r, c);
                let neighbors = self.world.get_neighbors(curr_coord);
                let num_of_alive_neighbors: usize =
                    Simulation::count_alive_neighbors(&neighbors, &grid_state);
                self.apply_conway_rule(&curr_coord, num_of_alive_neighbors, &grid_state);
            }
        }
    }
}

impl State for Simulation {
    fn step(&mut self, _: &mut Window) {
        self.evolve();
    }
}

fn main() {
    let mut window = Window::new("Game of life");

    window.set_light(Light::StickToCamera);
    let board: Box<Board> = Box::new(Board::new(
        &mut window,
        1000,
        1000,
        20.0,
        20.0,
        Vector3::<f32>::new(226.0 / 255.0, 135.0 / 255.0, 67.0 / 255.0),
        Vector3::<f32>::new(6.0 / 255.0, 57.0 / 255.0, 112.0 / 255.0),
    ));

    let mut initial_configuration: Vec<Coords2D> = vec![];
    let mut rng = rand::thread_rng();
    for r in 0..1000 {
        for c in 0..1000 {
            let random_float = rng.gen_range(0.0..=1.0);
            if random_float > 0.6 {
                initial_configuration.push(Coords2D::new(r, c));
            }
        }
    }
    let simulation: Simulation = Simulation::new(board, initial_configuration);

    window.render_loop(simulation);
}

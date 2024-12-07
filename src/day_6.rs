const INPUTS_FOLDER: &str = "inputs/day_6";

use core::fmt;
use std::collections::HashMap;

use crate::generic;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
    Walked,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' | '>' | 'V' | '<' => Self::Guard,
            _ => panic!("bad char input for Tile"),
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, c: {})", self.row, self.col)
    }
}

impl Position {
    fn walk(&self, steps: usize, direction: Direction) -> Self {
        let mut new_position = self.clone();
        match direction {
            Direction::Up => new_position.row -= steps,
            Direction::Down => new_position.row += steps,
            Direction::Left => new_position.col -= steps,
            Direction::Right => new_position.col += steps,
        }
        return new_position
    }

    fn from_obstacle(row: usize, col: usize, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self{row: row + 1, col: col},
            Direction::Down => Self{row: row - 1, col: col},
            Direction::Left => Self{row: row, col: col + 1},
            Direction::Right => Self{row: row, col: col - 1},
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct DetailedTile {
    tile_type: Tile,
    approach_direction: Option<Direction>,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'V' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Bad char given for direction!"),
        }
    }
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        return [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
        ].iter().copied();
    }

    fn rotate_90_CW(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn rotate_90_CCW(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl From<&Vec<String>> for TileMap {
    fn from(value: &Vec<String>) -> Self {
        return Self{tiles: value.iter().map(|x| x.chars().map(|y| Tile::from(y)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>()};
    }
}

impl TileMap {
    fn get_guard_start(&self) -> (usize, usize) {
        let row: usize = self.tiles.iter().position(|x| x.contains(&Tile::Guard)).expect("Cant find guard row!");
        let col: usize = self.tiles[row].iter().position(|x| *x == Tile::Guard).expect("Cant find guard column");
        return (row, col);
    }

    fn get_path_to_obstacle_from_position(&self, pos: &Position, direction: &Direction) -> Vec<Tile> {
        self.get_path_to_obtacle(pos.row, pos.col, direction)
    }

    fn get_path_to_obtacle(&self, row: usize, col: usize, direction: &Direction) -> Vec<Tile> {
        let mut path: Vec<Tile>;
        path = match direction {
            Direction::Up => self.tiles[..row].iter().map(|x| x[col].clone()).collect::<Vec<Tile>>(),
            Direction::Down => self.tiles[(row+1)..].iter().map(|x| x[col].clone()).collect::<Vec<Tile>>(),
            Direction::Left => self.tiles[row][..col].to_vec(),
            Direction::Right => self.tiles[row][(col+1)..].to_vec(),
        };

        if *direction == Direction::Up || *direction == Direction::Left {
            path.reverse();
        }

        return path;
    }

    fn get_distance_to_obstacle(&self, row: usize, col: usize, direction: &Direction) -> (bool, usize) {
        // Returns the distance to the nearest obstacle and a bool if the guard is still on the map.
        let path: Vec<Tile> = self.get_path_to_obtacle(row, col, direction);

        // println!("\tPath is {:?}", path);

        if path.contains(&Tile::Obstacle) {
            return (true, path.iter().position(|x| *x == Tile::Obstacle).unwrap());
        } else {
            return (false, path.len());
        }
    }

    fn check_obstacle_between(&self, pos1: &Position, pos2: &Position) -> bool {
        let (start_pos, end_pos, direction) = sort_positions(pos1, pos2);
        let mut return_bool: bool = false;
        // println!("Checking obstacle between {:?} and {:?}", start_pos, end_pos);
        match direction {
            Direction::Up => panic!("shouldnt have received up!"),
            Direction::Down => return_bool = self.tiles[start_pos.row..(end_pos.row+1)].iter().map(|x| x[start_pos.col].clone()).collect::<Vec<Tile>>().contains(&Tile::Obstacle),
            Direction::Left => panic!("shouldnt have received left!"),
            Direction::Right => return_bool = self.tiles[start_pos.row][start_pos.col..(end_pos.col + 1)].to_vec().contains(&Tile::Obstacle),
        };


        return return_bool;
    }

    fn get_CW_guard_position(&self, start_position: &Position, direction: &Direction) -> Option<Position> {
        println!("\tget_CW_guard_position from {:?} @ {:?}", start_position, direction);
        let mut path = self.get_path_to_obstacle_from_position(&start_position, &direction.rotate_90_CW());
        println!("\t\tpath is {:?}", path);

        if path.contains(&Tile::Obstacle) {
            let distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
            println!("\t\twalking {} in direction {:?}", distance_to_obstacle, direction.rotate_90_CW());
            return Some(start_position.walk(distance_to_obstacle, direction.rotate_90_CW()));
        } else {
            return None;
        }
    }

    fn get_CCW_guard_position(&self, start_position: &Position, direction: &Direction) -> Option<Position> {
        println!("\tget_CCW_guard_position from {:?} @ {:?}", start_position, direction);
        let offset_position: Position = match direction {
            Direction::Up => start_position.walk(1, Direction::Left),
            Direction::Left => start_position.walk(1, Direction::Down),
            Direction::Down => start_position.walk(1, Direction::Right),
            Direction::Right => start_position.walk(1, Direction::Up),
        };
        println!("\t\toffset_position = {:?}", offset_position);
        let mut path = self.get_path_to_obstacle_from_position(&offset_position, &direction.reverse());
        println!("\t\tpath is {:?}", path);

        if path.contains(&Tile::Obstacle) {
            let distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
            println!("\t\twalking {} in direction {:?}", distance_to_obstacle, direction.reverse());
            return Some(start_position.walk(distance_to_obstacle + 1, direction.reverse()));
        } else {
            return None;
        }
    }

    fn get_adjacent_positions(&self, position: &Position) -> Vec<Position> {
        let mut adjacent_positions: Vec<Position> = Vec::new();

        if position.row > 0 {
            adjacent_positions.push(position.walk(1, Direction::Up));
        }
        if position.row < (self.tiles.len() - 1) {
            adjacent_positions.push(position.walk(1, Direction::Down));
        }
        if position.col > 0 {
            adjacent_positions.push(position.walk(1, Direction::Left));
        }
        if position.col < (self.tiles[0].len() - 1) {
            adjacent_positions.push(position.walk(1, Direction::Right));
        }

        return adjacent_positions;
    }

    fn get_all_obstruction_positions(&self) -> Vec<Position> {
        let mut obstruction_positions: Vec<Position> = Vec::new();
        for r in 0..self.tiles.len() {
            for c in 0..self.tiles[0].len() {
                if self.tiles[r][c] == Tile::Obstacle {
                    obstruction_positions.push(Position{ row: r, col: c });
                }
            }
        }

        return obstruction_positions;
    }

    fn get_all_obstruction_adjacents(&self) -> Vec<Position> {
        let mut obstruction_adjacents: Vec<Position> = Vec::new();
        for p in self.get_all_obstruction_positions() {
            obstruction_adjacents.append(&mut self.get_adjacent_positions(&p));
        }

        return obstruction_adjacents;
    }
}

fn sort_positions(pos1: &Position, pos2: &Position) -> (Position, Position, Direction) {
    if pos1.row == pos2.row {
        if pos1.col <= pos2.col {
            return (pos1.clone(), pos2.clone(), Direction::Right);
        } else {
            return (pos2.clone(), pos1.clone(), Direction::Right);
        }
    } else {
        if pos1.row <= pos2.row {
            return (pos1.clone(), pos2.clone(), Direction::Down);
        } else {
            return (pos2.clone(), pos1.clone(), Direction::Down);
        }
    }
}

fn get_new_guard_position(current_positions: &Vec<Position>) -> Position {
    if current_positions.len() < 3 {
        panic!("Only 3 guard positions given!");
    }

    let new_row: usize;
    let new_col: usize;

    if current_positions[0].row == current_positions[1].row {
        // first two are same row, so new must be 3rd.
        new_row = current_positions[2].row;
    } else if current_positions[0].row == current_positions[2].row {
        // first and third have same row, so new must be 2nd.
        new_row = current_positions[1].row;
    } else {
        new_row = current_positions[0].row;
    }

    if current_positions[0].col == current_positions[1].col {
        new_col = current_positions[2].col;
    } else if current_positions[0].col == current_positions[2].col {
        new_col = current_positions[1].col;
    } else {
        new_col = current_positions[0].col
    }

    return Position {row: new_row, col: new_col};
}

fn get_obstacle_from_guard_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position{row: position.row - 1, col: position.col},
        Direction::Down => Position{row: position.row + 1, col: position.col},
        Direction::Left => Position{row: position.row, col: position.col - 1},
        Direction::Right => Position{row: position.row, col: position.col + 1},
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let tile_map: TileMap = TileMap::from(&input_lines);

    let (guard_start_row, guard_start_col): (usize, usize) = tile_map.get_guard_start();
    let guard_start_direction = Direction::from(input_lines[guard_start_row].chars().nth(guard_start_col).unwrap());

    let mut guard_on_map: bool = true;
    let mut total_distance: usize = 0;
    let mut row: usize = guard_start_row;
    let mut col: usize = guard_start_col;
    let mut direction: Direction = guard_start_direction.clone();
    let mut new_distance: usize = 0;
    let mut tiles_walked: Vec<Vec<Tile>> = vec![vec![Tile::Empty; tile_map.tiles[0].len()]; tile_map.tiles.len()];

    let mut approached_obstructions: Vec<Vec<DetailedTile>> = vec![
        vec![DetailedTile{tile_type: Tile::Empty, approach_direction: None}; tile_map.tiles[0].len()]; tile_map.tiles.len()];


    println!("Starting at row: {}, col: {}", row, col);
    while guard_on_map {
        (guard_on_map, new_distance) = tile_map.get_distance_to_obstacle(row, col, &direction);
        total_distance += new_distance;
        match direction {
            Direction::Up => {
                for r in (row - new_distance)..row {
                    tiles_walked[r][col] = Tile::Walked;
                }
                row -= new_distance;
                if (guard_on_map) {
                    approached_obstructions[row - 1][col] = DetailedTile{tile_type: Tile::Obstacle, approach_direction: Some(direction.clone())};
                }
            }
            Direction::Down => {
                for r in (row..(row + new_distance + 1)) {
                    tiles_walked[r][col] = Tile::Walked;
                }
                row += new_distance;
                if (guard_on_map) {
                    approached_obstructions[row + 1][col] = DetailedTile{tile_type: Tile::Obstacle, approach_direction: Some(direction.clone())};
                }
            },
            Direction::Left => {
                for c in ((col - new_distance)..col) {
                    tiles_walked[row][c] = Tile::Walked;
                }
                col -= new_distance;
                if (guard_on_map) {
                    approached_obstructions[row][col - 1] = DetailedTile{tile_type: Tile::Obstacle, approach_direction: Some(direction.clone())};
                }
            },
            Direction::Right => {
                for c in (col..(col + new_distance + 1)) {
                    tiles_walked[row][c] = Tile::Walked;
                }
                col += new_distance;
                if (guard_on_map) {
                    approached_obstructions[row][col + 1] = DetailedTile{tile_type: Tile::Obstacle, approach_direction: Some(direction.clone())};
                }
            },
        };

        println!("Now at row: {}, col: {}, guard_on_map: {}", row, col, guard_on_map);
        // print_walked_map(&tiles_walked);

        direction = direction.rotate_90_CW();
    }

    if !part_2 {
        let mut count_walked: usize = 0;
        for r in tiles_walked {
            for c in r {
                if c == Tile::Walked {
                    count_walked += 1;
                }
            }
        }
        return count_walked;
    } else {
        let mut walked_positions: Vec<Position> = Vec::new();
        for r in 0..tiles_walked.len() {
            for c in 0..tiles_walked[0].len() {
                if tiles_walked[r][c] == Tile::Walked {
                    walked_positions.push(Position{row: r, col: c});
                }
            }
        }

        let mut new_obstacle_positions: Vec<Position> = Vec::new();

        for p in walked_positions {
            println!("Running loop for position: {:?}", p);
            let mut walked_map_count: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); tile_map.tiles[0].len()]; tile_map.tiles.len()];

            let mut new_tile_map = tile_map.clone();
            new_tile_map.tiles[p.row][p.col] = Tile::Obstacle;
            
            guard_on_map = true;
            row = guard_start_row;
            col = guard_start_col;
            direction = guard_start_direction.clone();

            let mut obstacle_adjacents: HashMap<Position, HashMap<Direction, bool>> = HashMap::new();
            let mut adjacent_hashmap: HashMap<Direction, bool> = HashMap::new();
            for d in Direction::iter() {
                adjacent_hashmap.insert(d, false);
            }
            for adj in new_tile_map.get_all_obstruction_adjacents() {
                obstacle_adjacents.insert(adj, adjacent_hashmap.clone());
            }
            
            let mut step_index = 0;
            let mut completed_loop = false;
            while guard_on_map && !completed_loop {
                (guard_on_map, new_distance) = new_tile_map.get_distance_to_obstacle(row, col, &direction);

                let current_position = Position{row: row, col: col};
                if obstacle_adjacents.contains_key(&current_position) {
                    // println!("obstacle adjacents contains key");
                    if *obstacle_adjacents.get_mut(&current_position).unwrap().get(&direction).unwrap() {
                        println!("\tHit {:?} twice!!", current_position);
                        completed_loop = true;
                    } else {
                        // println!("\tRemembering {:?}", current_position);
                        obstacle_adjacents.get_mut(&current_position).unwrap().insert(direction, true);
                    }
                }
                match direction {
                    Direction::Up => {
                        for r in (row - new_distance)..row {
                            walked_map_count[r][col].push(step_index);
                            step_index += 1;
                        }
                        row -= new_distance;
                    }
                    Direction::Down => {
                        for r in (row..(row + new_distance + 1)) {
                            walked_map_count[r][col].push(step_index);
                            step_index += 1;
                        }
                        row += new_distance;
                    },
                    Direction::Left => {
                        for c in ((col - new_distance)..col) {
                            walked_map_count[row][c].push(step_index);
                            step_index += 1;
                        }
                        col -= new_distance;
                    },
                    Direction::Right => {
                        for c in (col..(col + new_distance + 1)) {
                            walked_map_count[row][c].push(step_index);
                            step_index += 1;
                        }
                        col += new_distance;
                    },
                };
                direction = direction.rotate_90_CW()
            }

            if guard_on_map && completed_loop {
                new_obstacle_positions.push(p);
            }
        }

        println!("{:?}", new_obstacle_positions);



        return new_obstacle_positions.len();
    }
}

fn print_map(walked_map: &Vec<Vec<Tile>>, special: Tile) {
    let mut print_char = '.';
    let mut print_row: String;
    for r in walked_map {
        print_row = String::new();
        for c in r {
            if *c == special {
                print_char = 'X';
            } else {
                print_char = '.';
            }
            print_row.push(print_char);
        }
        print!("{}\n", print_row);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 41);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 4656);
    }

    #[test]
    fn test_example_2() {
        let input_lines: Vec<String> = generic::read_in_file((INPUTS_FOLDER.to_owned() + "/input_example_1.txt").as_str());
        let tile_map: TileMap = TileMap::from(&input_lines);
        let start_guard_position: Position = Position::from_obstacle(3, 2, &Direction::Up);
        let mut next_CW_guard_position: Option<Position> = tile_map.get_CW_guard_position(&start_guard_position, &Direction::Up);
        assert!(next_CW_guard_position == Some(Position{row: 4, col: 6}));
        let mut next_CW_guard_position: Option<Position> = tile_map.get_CW_guard_position(&next_CW_guard_position.unwrap(), &Direction::Right);
        assert!(next_CW_guard_position == Some(Position{row: 8, col: 6}));

        let mut start_guard_position: Position = Position{row: 8, col: 6};
        println!("Start CCW from position = {:?}", start_guard_position);
        let mut next_CCW_guard_position: Option<Position>;
        next_CCW_guard_position = tile_map.get_CCW_guard_position(&start_guard_position, &Direction::Down);
        println!("next_CCW_guard_position = {:?}", next_CCW_guard_position);
        assert!(next_CCW_guard_position == Some(Position{row: 4, col: 6}));
        next_CCW_guard_position = tile_map.get_CCW_guard_position(&next_CCW_guard_position.unwrap(), &Direction::Right);
        println!("next_CCW_guard_position = {:?}", next_CCW_guard_position);
        assert!(next_CCW_guard_position == Some(Position{row: 4, col: 2}));

        start_guard_position = Position{row: 1, col: 8};
        println!("Start CCW from position = {:?}", start_guard_position);
        next_CCW_guard_position = tile_map.get_CCW_guard_position(&start_guard_position, &Direction::Right);
        println!("next_CCW_guard_position = {:?}", next_CCW_guard_position);
        assert!(next_CCW_guard_position == Some(Position{row: 1, col: 4}));
        next_CCW_guard_position = tile_map.get_CCW_guard_position(&next_CCW_guard_position.unwrap(), &Direction::Right);
        println!("next_CCW_guard_position = {:?}", next_CCW_guard_position);
        assert!(next_CCW_guard_position == Some(Position{row: 4, col: 2}));
    }

    #[test]
    fn walking() {
        let mut guard_position: Position = Position{row: 0, col: 0};
        assert!(guard_position.walk(5, Direction::Right) == Position{row: 0, col: 5});
        assert!(guard_position.walk(5, Direction::Down) == Position{row: 5, col: 0});
        
        guard_position = Position{row: 5, col: 5};
        assert!(guard_position.walk(5, Direction::Left) == Position{row: 5, col: 0});
        assert!(guard_position.walk(5, Direction::Up) == Position{row: 0, col: 5});
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 6);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1575);
        // 157 too low
    }
}
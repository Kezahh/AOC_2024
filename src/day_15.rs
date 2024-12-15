const INPUTS_FOLDER: &str = "inputs/day_15";

use crate::generic;
use crate::generic::{Direction, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => Self::Empty,
        }
    }
}


impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '>' => Self::Right,
            '<' => Self::Left,
            _ => panic!("Bad char given for direction!"),
        }
    }
}

#[derive(Debug, Clone)]
struct TileMap {
    tiles: Vec<Vec<Tile>>,
    robot: Position,
}

impl From<&[String]> for TileMap {
    fn from(value: &[String]) -> Self {
        return Self {
            tiles: value.iter().map(|x| x.chars().map(|y| Tile::from(y)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>(),
            robot: Position{row: 0,col: 0},
        }
    }
}

impl TileMap {
    fn get_tile(&self, row: usize, col: usize) -> Tile {
        return self.tiles[row][col];
    }

    fn set_tile(&mut self, row: usize, col: usize, value: Tile) {
        self.tiles[row][col] = value;
    }

    fn get_tile_from_position(&self, p: Position) -> Tile {
        return self.tiles[p.row][p.col];
    }

    fn set_tile_from_position(&mut self, p: Position, value: Tile) {
        self.tiles[p.row][p.col] = value;
    }

    fn swap_tiles(&mut self, p1: Position, p2: Position) {
        println!("\tSwapping {:?} with {:?}", p1, p2);
        println!("\tp1 = {:?}, p2 = {:?}", self.get_tile_from_position(p1), self.get_tile_from_position(p2));
        let temp: Tile = self.get_tile_from_position(p2);
        println!("\ttemp = {:?}", temp);
        self.set_tile_from_position(p2, self.get_tile_from_position(p1));
        self.set_tile_from_position(p1, temp);
    }

    fn get_robot_initial(&self) -> Position {
        for r in 0..self.tiles.len() {
            for c in 0..self.tiles[0].len() {
                if self.get_tile(r, c) == Tile::Robot {
                    return Position{row: r, col: c};
                }
            }
        }

        return Position{row: 0, col: 0};
    }

    fn move_robot(&mut self, direction: Direction) {
        let neighbour = self.get_neighbour(self.robot, direction);
        if neighbour.is_some() {
            let (next_position, next_tile) = neighbour.unwrap();
            match next_tile {
                Tile::Wall => (),
                Tile::Empty | Tile::Robot => self.robot = self.robot.walk(1, direction),
                Tile::Box => {
                    let path: Vec<Tile> = self.get_path_to_wall(self.robot.row, self.robot.col, &direction);
                    if path.contains(&Tile::Empty) {
                        let distance_to_tile: usize = path.iter().position(|x| *x == Tile::Empty).unwrap();
                        let target_position: Position = self.robot.walk(distance_to_tile + 1, direction);
                        self.swap_tiles(next_position, target_position);
                        self.robot = self.robot.walk(1, direction);
                    }
                },
            }
        }
    }

    fn get_neighbour(&self, p: Position, d: Direction) -> Option<(Position, Tile)> {
        let mut new_row: i32 = p.row as i32;
        let mut new_col: i32 = p.col as i32;
        match d {
            Direction::Up => new_row -= 1,
            Direction::Down => new_row += 1,
            Direction::Left => new_col -= 1,
            Direction::Right => new_col += 1,
        }

        if new_row < 0 || new_col < 0 {
            return None;
        } else {
            return Some((Position{row: new_row as usize, col: new_col as usize}, self.get_tile(new_row as usize, new_col as usize)));
        }
    }

    fn get_path_to_wall(&self, row: usize, col: usize, direction: &Direction) -> Vec<Tile> {
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

        return path[..path.iter().position(|x| *x == Tile::Wall).unwrap()].iter().copied().collect::<Vec<Tile>>();
    }

    fn print_map(&self) {
        for r in 0..self.tiles.len() {
            let mut row_string: String = String::new();
            for c in 0..self.tiles[0].len() {
                if r == self.robot.row && c == self.robot.col {
                    row_string.push('@');
                } else {
                    match self.get_tile(r, c) {
                        Tile::Wall => row_string.push('#'),
                        Tile::Box => row_string.push('O'),
                        Tile::Robot | Tile::Empty => row_string.push('.'),
                    }
                }
            }
            println!("{}", row_string);
        }
    }

    fn get_gps(&self) -> usize {
        let mut gps_sum: usize = 0;
        for r in 0..self.tiles.len() {
            for c in 0..self.tiles[0].len() {
                if self.get_tile(r, c) == Tile::Box {
                    gps_sum += (100*r) + c;
                }
            }
        }

        return gps_sum;
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let blank_line_index: usize = input_lines.iter().position(|x| x == "").unwrap();

    let mut tile_map: TileMap = TileMap::from(&input_lines[..blank_line_index]);
    tile_map.robot = tile_map.get_robot_initial();

    let mut directions: Vec<Direction> = Vec::new();
    for i in (blank_line_index + 1)..input_lines.len() {
        directions.append(&mut input_lines[i].chars().map(|x| Direction::from(x)).collect::<Vec<Direction>>());
    }

    for d in directions {
        println!("Moving in direction {:?}", d);
        tile_map.move_robot(d);
        // tile_map.print_map();
        println!("Done");
    }


    
    return tile_map.get_gps();
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
        assert!(answer == 10092);
    }

    #[test]
    fn example_1_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2028);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1456590);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}
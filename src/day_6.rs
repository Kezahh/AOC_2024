const INPUTS_FOLDER: &str = "inputs/day_6";

use core::fmt;

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

#[derive(Clone)]
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
}

#[derive(Debug, PartialEq, Clone)]
struct DetailedTile {
    tile_type: Tile,
    approach_direction: Option<Direction>,
}

#[derive(Debug, PartialEq, Clone)]
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
    fn rotate_90_CW(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

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


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let tile_map: TileMap = TileMap::from(&input_lines);

    let (guard_start_row, guard_start_col): (usize, usize) = tile_map.get_guard_start();
    let guard_start_direction = Direction::from(input_lines[guard_start_row].chars().nth(guard_start_col).unwrap());

    let mut guard_on_map: bool = true;
    let mut total_distance: usize = 0;
    let mut row: usize = guard_start_row;
    let mut col: usize = guard_start_col;
    let mut direction: Direction = guard_start_direction;
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
        let mut new_obstacle_map: Vec<Vec<Tile>> = vec![vec![Tile::Empty; tile_map.tiles[0].len()]; tile_map.tiles.len()];
        let mut current_tile: DetailedTile;
        for row in 0..approached_obstructions.len() {
            for col in 0..approached_obstructions[0].len() {
                current_tile = approached_obstructions[row][col].clone();
                if current_tile.tile_type == Tile::Obstacle {
                    println!("Working on DetailedTile at row: {}, col: {}, approach_direction: {:?}", row, col, current_tile.approach_direction);
                    // These are the positions of the Guard just in front of the obstacle.
                    let top_guard_position: Position;
                    let mut right_guard_position: Position = Position{row: 0, col: 0};
                    let mut left_guard_position: Position = Position{row: 0, col: 0};
                    let down_guard_position1: Position;
                    let down_guard_position2: Position;

                    let left_guard_position_proposed: Position;
                    let right_guard_position_proposed: Position;
                    let down_guard_position_proposed: Position;

                    match current_tile.approach_direction.unwrap() {
                        Direction::Up => {
                            let mut right_obstacle: bool = false;
                            let mut left_obstacle: bool = false;
                            let mut distance_to_obstacle: usize = 0;

                            top_guard_position = Position{row: row + 1, col: col};
                            
                            // Check to the right
                            let mut path = tile_map.get_path_to_obstacle_from_position(&top_guard_position, &Direction::Right);
                            println!("\tPath to the right is {:?}", path);

                            if path.contains(&Tile::Obstacle) {
                                distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
                                right_guard_position = top_guard_position.walk(distance_to_obstacle, Direction::Right);
                                right_obstacle = true;
                                println!("\tRight guard position = {:?}", right_guard_position);
                                // check if also down
                                path = tile_map.get_path_to_obstacle_from_position(&right_guard_position, &Direction::Down);
                                println!("\tPath to down is {:?}", path);
                                if path.contains(&Tile::Obstacle) {
                                    distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
                                    down_guard_position1 = right_guard_position.walk(distance_to_obstacle, Direction::Down);
                                    println!("\tDown guard position = {:?}", down_guard_position1);
                                    left_guard_position_proposed = Position{row: down_guard_position1.row, col: top_guard_position.col};

                                    if !tile_map.check_obstacle_between(&left_guard_position_proposed, &top_guard_position) {
                                        new_obstacle_map[left_guard_position_proposed.row][left_guard_position_proposed.col - 1] = Tile::Obstacle;
                                        println!("\tAdding new Obstacle at row: {}, col: {}", left_guard_position_proposed.row, left_guard_position_proposed.col - 1);
                                    }
                                }
                            }
                            
                            // Check to the left
                            path = tile_map.get_path_to_obtacle(top_guard_position.row, top_guard_position.col - 1, &Direction::Down);
                            println!("\tPath to the left is {:?}", path);

                            if path.contains(&Tile::Obstacle) {
                                distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
                                left_guard_position = top_guard_position.walk(distance_to_obstacle + 1, Direction::Down);
                                left_obstacle = true;
                                println!("\tLeft guard position = {:?}", left_guard_position);
                                // check if also down
                                path = tile_map.get_path_to_obtacle(left_guard_position.row + 1, left_guard_position.col, &Direction::Right);
                                println!("\tPath to down is {:?}", path);
                                if (path.contains(&Tile::Obstacle)) {
                                    distance_to_obstacle = path.iter().position(|x| *x == Tile::Obstacle).unwrap();
                                    down_guard_position2 = left_guard_position.walk(distance_to_obstacle + 1, Direction::Right);
                                    println!("\tDown guard position = {:?}", down_guard_position2);

                                    right_guard_position_proposed = Position{row: top_guard_position.row, col: down_guard_position2.col};

                                    if !tile_map.check_obstacle_between(&right_guard_position_proposed, &top_guard_position) {
                                        new_obstacle_map[right_guard_position_proposed.row][right_guard_position_proposed.col + 1] = Tile::Obstacle;
                                        println!("\tAdding new Obstacle at row: {}, col: {}", right_guard_position_proposed.row, right_guard_position_proposed.col - 1);
                                    }
                                }
                            }

                            if (left_obstacle && right_obstacle) {
                                down_guard_position_proposed = Position{row: left_guard_position.row, col: right_guard_position.col};

                                if !tile_map.check_obstacle_between(&down_guard_position_proposed, &left_guard_position) &&
                                        tile_map.check_obstacle_between(&down_guard_position_proposed, &right_guard_position) {
                                    new_obstacle_map[down_guard_position_proposed.row + 1][down_guard_position_proposed.col] = Tile::Obstacle;
                                    println!("\tAdding new Obstacle at row: {}, col: {}", down_guard_position_proposed.row + 1, down_guard_position_proposed.row + 1);
                                }
                                
                            }
                        },
                        Direction::Down => {},
                        Direction::Left => {},
                        Direction::Right => {},
                    }
                }
            }
        }

        print_map(&new_obstacle_map, Tile::Obstacle);

        return 0;
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
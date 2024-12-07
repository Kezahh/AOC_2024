const INPUTS_FOLDER: &str = "inputs/day_6";

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

    fn get_distance_to_obstacle(&self, row: usize, col: usize, direction: &Direction) -> (bool, usize) {
        // Returns the distance to the nearest obstacle and a bool if the guard is still on the map.
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

        // println!("\tPath is {:?}", path);

        if path.contains(&Tile::Obstacle) {
            return (true, path.iter().position(|x| *x == Tile::Obstacle).unwrap());
        } else {
            return (false, path.len());
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
            }
            Direction::Down => {
                for r in (row..(row + new_distance + 1)) {
                    tiles_walked[r][col] = Tile::Walked;
                }
                row += new_distance
            },
            Direction::Left => {
                for c in ((col - new_distance)..col) {
                    tiles_walked[row][c] = Tile::Walked;
                }
                col -= new_distance
            },
            Direction::Right => {
                for c in (col..(col + new_distance + 1)) {
                    tiles_walked[row][c] = Tile::Walked;
                }
                col += new_distance
            },
        };

        println!("Now at row: {}, col: {}, guard_on_map: {}", row, col, guard_on_map);
        // print_walked_map(&tiles_walked);

        direction = direction.rotate_90_CW();
    }

    let mut count_walked: usize = 0;
    for r in tiles_walked {
        for c in r {
            if c == Tile::Walked {
                count_walked += 1;
            }
        }
    }


    return count_walked;
}

fn print_walked_map(walked_map: &Vec<Vec<Tile>>) {
    let mut print_char = '.';
    let mut print_row: String;
    for r in walked_map {
        print_row = String::new();
        for c in r {
            if *c == Tile::Walked {
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
pub fn run() {
    let input: Vec<&str> = INPUT.lines().collect();
    let grid = Grid::parse(&input);
    let res1 = grid.run_grid_and_count_trees(&Movement { right: 3, down: 1 });
    println!("Answer (part 1): {}", res1);

    let part2_movements = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res2: usize = part2_movements
        .iter()
        .map(|i| Movement::from_tuple(i))
        .map(|m| grid.run_grid_and_count_trees(&m))
        .product();
    println!("Answer (part 2): {}", res2);
}

struct Position {
    pub x: usize,
    pub y: usize,
}

struct Movement {
    right: usize,
    down: usize,
}

impl Movement {
    fn from_tuple(input: &(usize, usize)) -> Movement {
        Movement {
            right: input.0,
            down: input.1,
        }
    }
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn run_grid_and_count_trees(&self, movement: &Movement) -> usize {
        let mut pos: Position = Position::initial_position();
        let mut count: usize = 0;
        loop {
            if self.is_tree(&pos) {
                count += 1;
            }
            let next_pos = pos.next_position(movement, self);
            match next_pos {
                Some(n) => pos = n,
                None => return count,
            }
        }
    }

    fn is_tree(&self, pos: &Position) -> bool {
        self.grid[pos.y][pos.x]
    }

    fn parse(lines: &Vec<&str>) -> Grid {
        let grid: Vec<Vec<bool>> = lines
            .iter()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        Grid { grid }
    }
}

impl Position {
    fn next_position(&self, movement: &Movement, grid: &Grid) -> Option<Position> {
        let maybe_y = self.y + movement.down;
        grid.grid.get(maybe_y).map(|x_grid| Position {
            x: (self.x + movement.right) % x_grid.len(),
            y: maybe_y,
        })
    }

    fn initial_position() -> Position {
        Position { x: 0, y: 0 }
    }
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day3::{Grid, Movement};

    const TESTINPUT: &str = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

    #[test]
    fn test_case() {
        let input: Vec<&str> = TESTINPUT.lines().collect();
        let grid = Grid::parse(&input);
        let movement = Movement { right: 3, down: 1 };
        let count = grid.run_grid_and_count_trees(&movement);
        assert_eq!(count, 7);
    }
}

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn go_in_direction(&mut self, direction: &Direction) {
        match direction {
            &Direction::Up => {
                self.y -= 1;
            },
            &Direction::Right => {
                self.x += 1;
            },
            &Direction::Left => {
                self.x -= 1;
            },
            &Direction::Down => {
                self.y += 1;
            },
        }
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    //This is used to rotate the guards position
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => {Direction::Right},
            Direction::Right => {Direction::Down},
            Direction::Down => {Direction::Left},
            Direction::Left => {Direction::Up}
        }
    }
}

fn print_map(map: Vec<Vec<char>>){
    let _ = map.into_iter().for_each(|f|{
        let new_string: String = f.into_iter().collect();
        println!("{}", new_string);
    });
}

fn move_guard( map:&mut Vec<Vec<char>>, mut guard_pos: Point, mut guard_direction: Direction, mut answer: i32) -> i32{
    while guard_pos.x >= 0 && guard_pos.y >= 0 && guard_pos.y < map.len() as i32 && guard_pos.x < map[0].len() as i32{
        let mut future_guard_position: Point = guard_pos.clone();
        future_guard_position.go_in_direction(&guard_direction);
        if !(future_guard_position.x >= 0 && future_guard_position.y >= 0 && future_guard_position.y < map.len() as i32 && future_guard_position.x < map[0].len()as i32){
            break
        }
        //If we have an obsticle
        if map[future_guard_position.y as usize][future_guard_position.x as usize] == '#'{
            guard_direction = guard_direction.turn_right();
            future_guard_position = guard_pos.clone();
            future_guard_position.go_in_direction(&guard_direction);
        }

            if map[future_guard_position.y as usize][future_guard_position.x as usize] != '*'{
                answer += 1;
                map[future_guard_position.y as usize][future_guard_position.x as usize] = '*';
            }

        guard_pos = future_guard_position;
    }
    answer
}

fn main() {
    let file = File::open("maze.txt").expect("Unable to find file called 'lists.txt'");
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        //We need to split the line up into a vector of ints
        let line: Vec<char> = line.unwrap().chars().collect();
        map.push(line);
    }
    //Find where the guard's position is!
    let mut guard_pos: Point = Point {x:0, y:0};
    for y in map.clone().into_iter().enumerate() {
        for x in y.1.into_iter().enumerate() {
            if map[x.0][y.0] == '^'{
                guard_pos = Point {x: y.0 as i32, y: x.0 as i32};
            }
        }
    }
    let initial_direction = Direction::Up;

    //Initially give it a 1, as the player counts as a position!
    let answer: i32 = move_guard(map.as_mut(), guard_pos, initial_direction, 1);
    println!("{}", answer);


}

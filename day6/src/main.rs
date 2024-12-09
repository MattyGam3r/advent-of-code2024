use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct MapPoint {
    location: Point,
    direction: Direction,
}

impl Point {
    fn go_in_direction(&mut self, direction: &Direction) {
        match direction {
            &Direction::Up => {
                self.y -= 1;
            }
            &Direction::Right => {
                self.x += 1;
            }
            &Direction::Left => {
                self.x -= 1;
            }
            &Direction::Down => {
                self.y += 1;
            }
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
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
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn print_map(map: Vec<Vec<char>>) {
    let _ = map.into_iter().for_each(|f| {
        let new_string: String = f.into_iter().collect();
        println!("{}", new_string);
    });
}

fn move_guard(
    mut map: Vec<Vec<char>>,
    mut guard_pos: Point,
    mut guard_direction: Direction,
    mut answer: i32,
    visited_locations: &mut HashSet<MapPoint>,
) -> i32 {
    while guard_pos.x >= 0
        && guard_pos.y >= 0
        && guard_pos.y < map.len() as i32
        && guard_pos.x < map[0].len() as i32
    {
        let mut future_guard_position: Point = guard_pos.clone();
        future_guard_position.go_in_direction(&guard_direction);
        if !(future_guard_position.x >= 0
            && future_guard_position.y >= 0
            && future_guard_position.y < map.len() as i32
            && future_guard_position.x < map[0].len() as i32)
        {
            break;
        }
        //If we have an obsticle
        if map[future_guard_position.y as usize][future_guard_position.x as usize] == '#' {
            guard_direction = guard_direction.turn_right();
            future_guard_position = guard_pos.clone();
            future_guard_position.go_in_direction(&guard_direction);
        }

        if map[future_guard_position.y as usize][future_guard_position.x as usize] != '*' {
            answer += 1;
            map[future_guard_position.y as usize][future_guard_position.x as usize] = '*';
            visited_locations.insert(MapPoint { location: future_guard_position, direction: guard_direction });
        }

        guard_pos = future_guard_position;
    }
    answer
}

fn detect_loop( mut map: Vec<Vec<char>>, mut guard_pos: Point, mut guard_direction: Direction) -> bool {
    let mut visited_locations: HashSet<MapPoint> = HashSet::new();
    while guard_pos.x >= 0 && guard_pos.y >= 0 && guard_pos.y < map.len() as i32 && guard_pos.x < map[0].len() as i32
    {
    let mut future_guard_position: Point = guard_pos.clone();
    future_guard_position.go_in_direction(&guard_direction);
    if !(future_guard_position.x >= 0 && future_guard_position.y >= 0 && future_guard_position.y < map.len() as i32 && future_guard_position.x < map[0].len() as i32)
    {
         break;
    }
    //If we have an obsticle, fix the position
    if map[future_guard_position.y as usize][future_guard_position.x as usize] == '#' {
        guard_direction = guard_direction.turn_right();
        future_guard_position = guard_pos.clone();
        //future_guard_position.go_in_direction(&guard_direction);
    }
    if map[future_guard_position.y as usize][future_guard_position.x as usize] != '*' {
        map[future_guard_position.y as usize][future_guard_position.x as usize] = '*';
    }
    if visited_locations.contains(&MapPoint { location: future_guard_position, direction: guard_direction }){
        //print_map(map);
        //println!("w'");
        return true;
    }
    visited_locations.insert(MapPoint { location: future_guard_position, direction: guard_direction });


    guard_pos = future_guard_position;
}
    return false;
}

fn main() {
    let file = File::open("maze.txt").expect("Unable to find file called 'lists.txt'");
    let reader = BufReader::new(file);
    let mut locations_set: HashSet<MapPoint> = HashSet::new();
    
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        //We need to split the line up into a vector of ints
        let line: Vec<char> = line.unwrap().chars().collect();
        map.push(line);
    }
    //Find where the guard's position is!
    let mut guard_pos: Point = Point { x: 0, y: 0 };
    for y in map.clone().into_iter().enumerate() {
        for x in y.1.into_iter().enumerate() {
            if map[x.0][y.0] == '^' {
                guard_pos = Point {
                    x: y.0 as i32,
                    y: x.0 as i32,
                };
            }
            
        }
    }
    let initial_direction = Direction::Up;
    locations_set.insert(MapPoint { location: guard_pos, direction: initial_direction });

    //Initially give it a 1, as the player counts as a position!
    let answer: i32 = move_guard(map.clone(), guard_pos, initial_direction.clone(), 1, &mut locations_set);
    println!("{}", answer);
    let mut p2ans = 0;
    //PART 2
    //For every position visited, we will add an obsticle, then determine if the new map has a cycle or not with that obstacle
    for y in map.clone().into_iter().enumerate() {
        for x in y.1.into_iter().enumerate() {
            let mut new_map = map.clone();
            if new_map[x.0][y.0] != '^' &&  new_map[x.0][y.0] != '#' {
                new_map[x.0][y.0] = '#';
                if detect_loop(new_map, guard_pos, initial_direction) {
                    p2ans+=1;
                    //println!("{}", p2ans);
                }
            }

        }
    }
    //279 too low
    //4372 too high
    // 1614 too high
    // 1615 not correct
    // 1616 just not correct
    //1705 correct
    println!("P2 ans: {}", p2ans);
}

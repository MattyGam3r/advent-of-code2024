use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use core::ops::Sub;
use core::ops::Add;
#[derive(PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn get_area_around(point: Point) -> Vec<Point> {
    let mut return_val: Vec<Point> = Vec::new();
    let mut new_x = point.x - 1;
    for _i in 1..=3{
        let mut new_y = point.y - 1;
        for _j in 1..=3{
            let new_point = Point {x: new_x.clone(), y: new_y.clone()};
            if new_point != point{
                return_val.push(new_point);
            }
            new_y+=1;
        }
        new_x+=1;
    }
    return_val
}

fn search_for_character(cur_val: char, word_map: &Vec<String>, point: Point, prev_point: Point) -> i32{
    if point.x < 0 || point.x >= (word_map.len()).try_into().unwrap() || point.y < 0 || point.y >= (word_map[point.x as usize].len()).try_into().unwrap(){
        return 0;
    }
    
    let search_vec = get_area_around(point);
    let next_char: char;
    if cur_val == word_map[point.y.clone() as usize].chars().nth(point.x.clone() as usize).unwrap(){
        println!("CURRENT VAL: {:?} with CHAR: {}", point, cur_val);
        match cur_val {
            'S' => return 1,
            'X' => next_char = 'M',
            'M' => next_char = 'A',
            'A' => next_char = 'S',
            _ => return 0,
        }

        if cur_val == 'X'{
            return search_vec.into_iter().fold(0, |acc,p|{
                //println!("Looking for {} with point {:?}", next_char, p);
                acc + search_for_character(next_char, word_map, p, point)
            });
        }
        else{
            let new_point = (point - prev_point) + point;
            println!("GOING TO {:?} NEXT", new_point);
            return search_for_character(next_char, word_map, new_point, point);
        }

    }

    return 0;

}


fn main() {
    let file = File::open("words.txt").expect("Unable to find file called 'words.txt'");
    let reader = BufReader::new(file);

    //Parsing input into a way we can access later on
    let mut word_map: Vec<String> = Vec::new();
    //Parse the data into a 2x2 matrix
    for row in reader.lines(){
        word_map.push(row.unwrap());
    }
    let mut answer: i32 = 0;

    for row in word_map.iter().enumerate(){
        for col in word_map[row.0].chars().enumerate() {
            if 'X' == word_map[row.0.clone() as usize].chars().nth(col.0.clone() as usize).unwrap(){
                answer += search_for_character('X', &word_map, Point{x: col.0 as i32, y: row.0 as i32},Point{x: col.0 as i32, y: row.0 as i32});
            }
            
        }
    }

    println!("{}", answer);
    //Now! It's time for our solution
    //490 is too low
    //51786 too high

    let point = Point {x : 3, y: 3};
    let around = get_area_around(point);
    println!("{:?} {:?}", point, around);
}

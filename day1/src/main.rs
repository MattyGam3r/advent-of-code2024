use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file = File::open("lists.txt").expect("Unable to find file called 'lists.txt'");
    let reader = BufReader::new(file);


    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in reader.lines(){
        //We need to split the line up into a vector of ints
        let line: Vec<String> = line.unwrap().split(' ').filter_map(|f| {
            match f {
                "" => {
                    None
                },
                _ => {
                    Some(f.to_string())
                }
            }
        }).collect();
        list1.push(line[0].parse::<i32>().unwrap());
        list2.push(line[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    
    let result = list1.iter().zip(list2.iter()).fold(0, |acc, x| acc + (x.0 - x.1).abs());
    println!("PART 1 RESULT: {}", result);

    //Part 2
    //We need to make a map and store the number of times a certain thingie happens
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    list2.iter().for_each(|val| {
        match freq_map.contains_key(val) {
            true => {freq_map.insert(val.clone(), freq_map[val] + 1);},
            false => {freq_map.insert(val.clone(), 1);},
        };
    });

    let result_part_2 = list1.iter().fold(0, |acc, x| {
        match freq_map.contains_key(x) {
            true => {
                acc + (freq_map[x] * x)
            },
            false => { acc + 0 },
        }
    });
    println!("{}", result_part_2);
    //Now we have a frequency hashmap, we just gotta
}

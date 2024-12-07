use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq)]
enum ReadState {
    Dictionary,
    Conditions,
}

fn main() {
    let file = File::open("queue.txt").expect("Unable to find file called 'queue.txt'");
    let reader = BufReader::new(file);
    let mut current_state = ReadState::Dictionary;
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut conditions: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines(){
        //We need to split the line up into a vector of ints
        if line.as_ref().unwrap().len() == 0 {
            current_state = ReadState::Conditions;
            continue;
        }
        
        if current_state == ReadState::Dictionary {
            let line: Vec<String> = line.unwrap().split('|').map(|f| f.to_string()).collect();
            let num1 = line[0].parse::<i32>().unwrap();
            let num2 = line[1].parse::<i32>().unwrap();
    
            match rules.contains_key(&num1) {
                true => {
                    rules.get_mut(&num1).unwrap().insert(num2);
                },
                false => {
                    let mut new_hashset: HashSet<i32> = HashSet::new();
                    new_hashset.insert(num2);
                    rules.insert(num1, new_hashset);
                }
            }
        }
        else{
            let line: Vec<i32> = line.unwrap().split(',').map(|f| f.parse::<i32>().unwrap()).collect();
            conditions.push(line);
        }
        
    }
    
    //Now~! Time to finish this
    let mut answer : i32 = 0;
    let mut bad_conditions: Vec<Vec<i32>> = Vec::new();
    for condition in conditions.clone() {
        let mut fulfilled = true;
        for num in 0..condition.len(){
            
            for future_num in num..condition.len(){
                if rules.contains_key(&(condition[future_num] as i32)){
                    if rules[&(condition[future_num] as i32)].contains(&(condition[num] as i32)){
                        //println!("NUM: {} is in FUTURE NUM: {} IN CONDITION ARRAY: {:?}", num, future_num, condition);
                        fulfilled = false;
                    }
                }
            }
        }
        if fulfilled == true {
            answer += condition[condition.len() / 2];
        }
        else{
            bad_conditions.push(condition);
        }
    }
    // 144354 too high
    // 5312 too high
    // 0 not correct
    println!("PART 1 ANS: {}", answer);
    let mut p2ans = 0;
    //Now, we need to fix the bad ones
    for mut bad_condition in bad_conditions.clone() {
        for num in 0..bad_condition.len(){
            for future_num in num..bad_condition.len(){
                if rules.contains_key(&(bad_condition[future_num] as i32)){
                    if rules[&(bad_condition[future_num] as i32)].contains(&(bad_condition[num] as i32)){
                        //println!("NUM: {} is in FUTURE NUM: {} IN CONDITION ARRAY: {:?}", num, future_num, bad_condition);
                        bad_condition.swap(num, future_num);
                    }
                }
            }
        }
        p2ans +=bad_condition[bad_condition.len() / 2];
    }
    //5263 is too high
    println!("P2 ANS: {}", p2ans);
}

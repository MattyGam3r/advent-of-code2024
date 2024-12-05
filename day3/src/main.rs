use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;


fn main() {
    let file = File::open("input.txt").expect("Unable to find file called 'lists.txt'");
    let reader = BufReader::new(file);
    let regex: Regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut answer = 0;
    let mut add = true;
    for line in reader.lines(){
        
        
        answer += Regex::find_iter(&regex, line.unwrap().as_str()).fold(0, |acc, x|{
            println!("{}", x.as_str());
            match x.as_str() {
                "don't()" => {
                    add = false;
                    acc + 0
                },
                "do()" => {
                    add = true;
                    acc + 0
                }
                _ => {
                    if add == true{
                        let nums: Vec<&str> = Regex::find_iter(&Regex::new(r"[0-9]{1,3}").unwrap(), x.as_str()).map(|f| f.as_str()).collect();
                        println!("{}", x.as_str());
                        let num1 = nums[0].parse::<i32>().expect("Unable to unwrap");
                        let num2 = nums[1].parse::<i32>().expect("Unable");
                        println!("{} {}", num1, num2);
                        acc + (num1 * num2)
                    }
                    else{
                        acc + 0
                    }

                }
            }
            //We need to turn the mul(x,y) into the two numbers
        });

    }
    //Now we have a frequency hashmap, we just gotta
    println!("Part 1: {}", answer);
}

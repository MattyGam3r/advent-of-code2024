use std::cmp::Ordering;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

enum ReportType {
    Increasing,
    Decreasing,
    Unknown,
}

enum Validity {
    Valid,
    NonValid,
}


fn main() {
    //Loop through ALL reports (lines)
    //A report contains levels
    //Levels must be either dereasing or increasing, AND
    //Any two adjacent levels must differ by at least one and at most 3
    //Return how many reports are safe

    let file = File::open("report.txt").expect("Unable to find file called 'lists.txt'");
    let reader = BufReader::new(file);
    let mut valid_lines = 0;
    let mut valid_lines_p2 = 0;
    for line in reader.lines(){
        //Dirty vector to hold stuff
        let mut current_line_vals: Vec<i32> = Vec::new();
        let mut current_report_type = ReportType::Unknown;
        let mut current_validity = Validity::Valid;
        let mut invalid_level_encountered = false;
        //A line contains integers.
        line.unwrap().split(' ').into_iter().for_each(|val| {
            let val = val.parse::<i32>().expect("Unable to Convert val into an Integer");
            let mut current_line_has_error = false;
            match current_line_vals.len() {
                0 => {},
                _ => {
                    match current_report_type {
                        ReportType::Decreasing => {
                            if current_line_vals.last().unwrap() - val > 3 || current_line_vals.last().unwrap() - val < 1 {
                                if invalid_level_encountered == true{
                                    current_validity = Validity::NonValid;
                                }
                                else{
                                    invalid_level_encountered = true;
                                    current_line_has_error = true;
                                    if current_line_vals.len() == 1 {
                                        current_report_type = ReportType::Unknown;
                                    }
                                }
                                
                            }
                        },

                        ReportType::Increasing => {
                            if val - current_line_vals.last().unwrap() > 3 || val - current_line_vals.last().unwrap() < 1 {
                                if invalid_level_encountered == true{
                                    current_validity = Validity::NonValid;
                                }
                                else{
                                    invalid_level_encountered = true;
                                    current_line_has_error = true;
                                    if current_line_vals.len() == 1 {
                                        current_report_type = ReportType::Unknown;
                                    }
                                }
                            }
                        },


                        ReportType::Unknown => {
                            match val.cmp(current_line_vals.last().unwrap()) {

                                Ordering::Less => {
                                    current_report_type = ReportType::Decreasing;
                                    if current_line_vals.last().unwrap() - val > 3 || current_line_vals.last().unwrap() - val < 1 {
                                        if invalid_level_encountered == true{
                                            current_validity = Validity::NonValid;
                                        }
                                        else{
                                            invalid_level_encountered = true;
                                            current_line_has_error = true;
                                            if current_line_vals.len() == 1 {
                                                current_report_type = ReportType::Unknown;
                                            }
                                        }
                                    }
                                },

                                Ordering::Equal => {
                                    if invalid_level_encountered == true{
                                        current_validity = Validity::NonValid;
                                    }
                                    invalid_level_encountered = true;
                                    current_line_has_error = true;
                                },

                                Ordering::Greater => {
                                    current_report_type = ReportType::Increasing;
                                    if val - current_line_vals.last().unwrap() > 3 || val - current_line_vals.last().unwrap() < 1 {
                                        if invalid_level_encountered == true{
                                            current_validity = Validity::NonValid;
                                        }
                                        else{
                                            invalid_level_encountered = true;
                                            current_line_has_error = true;
                                            if current_line_vals.len() == 1 {
                                                current_report_type = ReportType::Unknown;
                                            }
                                        }
                                    }
                                },
                            }
                        },
                    }
                }
                
            }
            //If the current value has an error, we don't look at it in the future again
            if current_line_has_error == false {
                current_line_vals.push(val);
            }
        });
        match current_validity {
            Validity::NonValid => {}
            Validity::Valid => {valid_lines_p2 += 1;}
        }
        match invalid_level_encountered {
            true => { },
            _ => {valid_lines+= 1;}
        }
    }
    println!("PART 1 (Number of Valid Reports): {}", valid_lines);
    println!("PART 2 (Number of Valid Reports): {}", valid_lines_p2);
}
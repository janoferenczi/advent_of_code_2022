use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file: &str = args.get(1).unwrap();

    let mut max_calories: i32 = 0;
    let mut current_sum: i32 = 0;
    let mut sum_per_elfs: Vec<i32> = Vec::new();

    
    for line in read_to_string(input_file).unwrap().lines(){
	if line.is_empty() {
	    if current_sum > max_calories {
		max_calories = current_sum;
	    }

	    sum_per_elfs.push(current_sum);
	    
	    current_sum = 0;
	} else {
	    let int_value: i32 = line.parse::<i32>().unwrap();
	    
	    current_sum += int_value;
	}
    }


    sum_per_elfs.sort_by(|a, b| b.cmp(a));

    let sum = sum_per_elfs.get(0).unwrap() + sum_per_elfs.get(1).unwrap() + sum_per_elfs.get(2).unwrap();
    
    
    println!("The total of the three most richest elfs calories is: {}", sum);
    println!("The max calories Elf has {} calories.", max_calories);
}

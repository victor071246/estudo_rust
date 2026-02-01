use std::{fs::File, io::Read};

fn read_file(filename: &str) -> Vec<i32>{
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let nums: Vec<i32> = contents.split_whitespace().map(|substring| substring.parse().expect("Unable to parse number")).collect();
    nums
}

fn sort(nums: &mut Vec<i32>){
    let i = 0;
    let mut j = nums.len() -1;
    while i < j {
        for k in i..j{
            if nums[k] > nums[k+1]{
                let temp = nums[k];
                nums[k] = nums[k+1];
                nums[k+1] = temp;
            }
        }

        j -= 1
    }
}

fn main() {
    let filename = "nums.txt";
    let mut nums = read_file(filename);
    sort(&mut nums);
    for num in nums{
        println!("{}", num);
    }
}

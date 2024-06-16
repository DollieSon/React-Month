use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub struct Point {
    pub x: u8,
    pub y: u8,
}
impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x: x, y: y }
    }
    pub fn distance(&self, towards: Point) -> Point {
        fn getdist(x1: u8, x2: u8) -> u8 {
            let res = (x1 as i16 - x2 as i16).abs();
            if res > u8::MAX as i16 {
                u8::MAX
            } else {
                res as u8
            }
        }
        Point {
            x: getdist(self.x, towards.x),
            y: getdist(self.y, towards.y),
        }
    }
}

fn main() {
    let home = Point::new(12, 0);
    let school = Point::new(21, 5);
    let distance_home_school = home.distance(school);
    println!("{} {}", home.x, home.y);
    println!("{} {}", distance_home_school.x, distance_home_school.y);
}

fn guessing_game() {
    println!("guess the wumber");
    // making the number
    let number = rand::thread_rng().gen_range(1..=100);
    println!("secret number is {}", number);
    'game_loop: loop {
        println!("enter a number:");
        let mut word = String::new();
        // reading
        io::stdin()
            .read_line(&mut word)
            .expect("something went wrong"); // error handling
        let word: u32 = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match word.cmp(&number) {
            Ordering::Less => println!("Too Small, Like your PP"),
            Ordering::Greater => println!("Too big, Like my PP"),
            Ordering::Equal => {
                println!("Good Job");
                break 'game_loop;
            }
        }
    }
}

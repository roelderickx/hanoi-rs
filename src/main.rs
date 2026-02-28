mod disc;
mod tower;
mod hanoi;

use std::io;
use hanoi::Hanoi;

fn input_tower(text: &str, allow_zero: bool) -> usize {
    loop {
        println!("{}", text);

        let mut tower = String::new();

        io::stdin()
            .read_line(&mut tower)
            .expect("Failed to read line");

        let tower: usize = match tower.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if (allow_zero || tower >= 1) && tower <= 3 {
            return tower;
        }
    }
}

fn main() {
    let mut h = Hanoi::create();
    h.print();

    h.choose_amount_discs();

    loop {
        h.print();
        let from_tower = input_tower("From tower number (stop = 0)", true);

        if from_tower == 0 {
            break;
        }
        else {
            let to_tower = input_tower("To tower number", false);

            h.move_disc(from_tower - 1, to_tower - 1);
        }
    }
    
    h.check_result();
}


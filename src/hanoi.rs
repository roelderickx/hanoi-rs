use crate::tower::Tower;

use std::io;

pub struct Hanoi {
    towers: [Tower; 3],
    turn: u8,
}

impl Hanoi {
    pub fn create() -> Self {
        Hanoi {
            turn: 1,
            towers: [
                Tower::create(7),
                Tower::create(0),
                Tower::create(0)
            ]
        }
    }
    
    pub fn choose_amount_discs(&mut self) {
        loop {
            println!("How many discs (min 3, max 7)");

            let mut amount = String::new();

            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");

            let amount: u8 = match amount.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            if amount >= 3 && amount <= 7 {
                self.towers[0].initialize(amount);
                self.towers[1].initialize(0);
                self.towers[2].initialize(0);
                break;
            }
        }
    }
    
    pub fn move_disc(&mut self, from_tower: usize, to_tower: usize) {
        // Valid move?
        // Top disc on to_tower must be larger than the one we're moving from from_tower
        let from_top_disc_size = self.towers[from_tower].get_top_disc_size();
        let to_top_disc_size = self.towers[to_tower].get_top_disc_size();
        
        if to_top_disc_size > 0 && from_top_disc_size > to_top_disc_size {
            println!("A larger disc cannot be placed on a smaller disc, move not allowed.");
            return;
        }
        else {
            self.towers[from_tower].remove_top_disc();
            self.towers[to_tower].add_disc(from_top_disc_size);
            self.turn += 1;
        }
    }
    
    pub fn check_result(&self) {
        let top_disc_size_1 = self.towers[0].get_top_disc_size();
        let top_disc_size_2 = self.towers[1].get_top_disc_size();

        if top_disc_size_1 == 0 && top_disc_size_2 == 0 {
            println!("Solution correct in {} moves", self.turn - 1);
        }
        else {
            println!("Incorrect solution, all discs must be moved to the third tower");
        }
    }

    pub fn print(&self) {
        println!("*** TOWERS OF HANOI ***");
        println!("Turn {}\n", self.turn);

        for i in (0..8).rev() {
            if i == 7 {
                // header: tower number
                let spaces6 = " ".repeat(6);
                let spaces13 = " ".repeat(13);
                println!("{}1{}2{}3\n", spaces6, spaces13, spaces13);
            }
            else {
                for j in 0..3 {
                    self.towers[j].print_disc(i);
                }
                println!("");
            }
        }
    }
}


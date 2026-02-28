#[derive(Copy,Clone)]
pub struct Disc {
    size: u8
}

impl Disc {
    pub fn create(size: u8) -> Self {
        Self { size }
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }
    
    pub fn set_size(&mut self, size: u8) {
        self.size = size;
    }

    pub fn print(&self) {
        if self.size < 2 {
            let spaces = " ".repeat(6);
            if self.size == 0 {
                print!("{}|{} ", spaces, spaces);
            }
            else if self.size == 1 {
                print!("{}●{} ", spaces, spaces);
            }
        }
        else {
            let amount_blocks: usize = ((self.size * 2) - 1).into();
            let amount_spaces: usize = (13 - amount_blocks) / 2;
            let blocks = "■".repeat(amount_blocks-2);
            let spaces = " ".repeat(amount_spaces);
            print!("{}◖{}◗{} ", spaces, blocks, spaces);
        }
    }
}


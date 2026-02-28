use crate::disc::Disc;

pub struct Tower {
    discs: [Disc; 7]
}

impl Tower {
    pub fn create(amount_discs: u8) -> Self {
        let discs = [ Disc::create(0); 7 ];
        let mut tower = Self { discs };
        tower.initialize(amount_discs);
        tower
    }
    
    pub fn initialize(&mut self, amount_discs: u8) {
        for i in 0..7 {
            if i <= amount_discs {
                self.discs[i as usize].set_size(amount_discs - i);
            }
            else {
                self.discs[i as usize].set_size(0);
            }
        }
    }

    pub fn get_top_disc_size(&self) -> u8 {
        for i in (0..7).rev() {
            if self.discs[i].get_size() > 0 {
                return self.discs[i].get_size();
            }
        }

        return 0;
    }
    
    pub fn remove_top_disc(&mut self) {
        for i in (0..7).rev() {
            if self.discs[i].get_size() > 0 {
                self.discs[i].set_size(0);
                break;
            }
        }
    }
    
    pub fn add_disc(&mut self, size: u8) {
        for i in (0..6).rev() {
            if self.discs[i].get_size() > 0 {
                self.discs[i+1].set_size(size);
                return;
            }
        }
        
        self.discs[0].set_size(size);
    }
    
    pub fn print_disc(&self, disc_number: usize) {
        self.discs[disc_number].print();
    }
}


#[derive(Serialize, Debug, Clone)]
pub struct BeetleGenome {
    genome: Vec<BeetleGene>,
}

impl BeetleGenome {
    pub fn new() -> BeetleGenome {
        BeetleGenome {
            genome: vec![
                BeetleGene::Size(0.5),
                BeetleGene::CarapaceDensity(0.5),
                BeetleGene::Strength(0.5),
                BeetleGene::Quickness(0.5),
                // other gene ideas:
                // venom
                // max health
                // coordination (affects turning speed, etc)
                // mandible size
                // mandible sharpness
                // mandible strength
            ],
        }
    }

    pub fn size(&self) -> f32 {
        match self.genome[0] {
            BeetleGene::Size(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_size(&mut self, value: f32) {
        self.genome[0] = BeetleGene::Size(value);
    }

    pub fn carapace_density(&self) -> f32 {
        match self.genome[1] {
            BeetleGene::CarapaceDensity(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_carapace_density(&mut self, carapace_density: f32) {
        self.genome[1] = BeetleGene::CarapaceDensity(carapace_density);
    }

    pub fn strength(&self) -> f32 {
        match self.genome[2] {
            BeetleGene::Strength(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_strength(&mut self, strength: f32) {
        self.genome[2] = BeetleGene::Strength(strength);
    }
    
    pub fn quickness(&self) -> f32 {
        match self.genome[3] {
            BeetleGene::Quickness(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_quickness(&mut self, quickness: f32) {
        self.genome[3] = BeetleGene::Quickness(quickness);
    }

    pub fn set_gene_value(&mut self, index: i32, value: f32) {
        match index {
            0 => {
                self.genome[0] = BeetleGene::Size(value);
            },
            1 => {
                self.genome[1] = BeetleGene::CarapaceDensity(value);
            },
            2 => {
                self.genome[2] = BeetleGene::Strength(value);
            },
            3 => {
                self.genome[3] = BeetleGene::Quickness(value);
            },
            _ => {
            }
        }
    }

    pub fn get_num_genes(&self) -> i32 {
        4
    }
}

// TODO: properly implement a Ratio type to use with these instead of f32
// should limit the values to 0.0-1.0
#[derive(Serialize, Debug, Clone)]
enum BeetleGene {
    Size(f32),
    CarapaceDensity(f32),
    Strength(f32),
    Quickness(f32),
}

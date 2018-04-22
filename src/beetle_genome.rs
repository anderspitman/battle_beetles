use rand::{Rng, thread_rng};

#[derive(Serialize, Debug, Clone)]
pub struct BeetleGenome {
    genome: Vec<BeetleGene>,
}

// TODO: properly implement a Ratio type to use with these instead of f32
// should limit the values to 0.0-1.0
#[derive(Serialize, Debug, Clone)]
enum BeetleGene {
    Size(f32),
    CarapaceDensity(f32),
    Strength(f32),
    Quickness(f32),
    Venomosity(f32),
    MandibleSharpness(f32),
}

pub enum BeetleGeneIndex {
    Size = 0,
    CarapaceDensity = 1,
    Strength = 2,
    Quickness = 3,
    Venomosity = 4,
    MandibleSharpness = 5,
}

impl From<i32> for BeetleGeneIndex {
    fn from(val: i32) -> BeetleGeneIndex {
        match val {
            0 => BeetleGeneIndex::Size,
            1 => BeetleGeneIndex::CarapaceDensity,
            2 => BeetleGeneIndex::Strength,
            3 => BeetleGeneIndex::Quickness,
            4 => BeetleGeneIndex::Venomosity,
            5 => BeetleGeneIndex::MandibleSharpness,
            _ => panic!("Invalid gene index {}", val)
        }
    }
}

impl BeetleGenome {
    pub fn new() -> BeetleGenome {
        BeetleGenome {
            genome: vec![
                BeetleGene::Size(0.5),
                BeetleGene::CarapaceDensity(0.5),
                BeetleGene::Strength(0.5),
                BeetleGene::Quickness(0.5),
                BeetleGene::Venomosity(0.5),
                BeetleGene::MandibleSharpness(0.5),
                // other gene ideas:
                // coordination (affects turning speed, etc)
                // mandible size
                // mandible strength
            ],
        }
    }

    pub fn get_num_genes() -> i32 {
        6
    }

    pub fn get_random_gene_index() -> BeetleGeneIndex {
        let num_genes = BeetleGenome::get_num_genes();
        let random_gene_index = thread_rng().gen_range::<i32>(0, num_genes);

        BeetleGeneIndex::from(random_gene_index)
    }

    pub fn set_random_genome(&mut self) {

        let mut rng = thread_rng();
        self.genome[BeetleGeneIndex::Size as usize] = BeetleGene::Size(rng.gen());
    }

    pub fn size(&self) -> f32 {
        match self.genome[BeetleGeneIndex::Size as usize] {
            BeetleGene::Size(value) => value,
            _ => panic!() 
        }
    }

    pub fn carapace_density(&self) -> f32 {
        match self.genome[BeetleGeneIndex::CarapaceDensity as usize] {
            BeetleGene::CarapaceDensity(value) => value,
            _ => panic!() 
        }
    }

    pub fn strength(&self) -> f32 {
        match self.genome[BeetleGeneIndex::Strength as usize] {
            BeetleGene::Strength(value) => value,
            _ => panic!() 
        }
    }
    
    pub fn quickness(&self) -> f32 {
        match self.genome[BeetleGeneIndex::Quickness as usize] {
            BeetleGene::Quickness(value) => value,
            _ => panic!() 
        }
    }

    pub fn venomosity(&self) -> f32 {
        match self.genome[BeetleGeneIndex::Venomosity as usize] {
            BeetleGene::Venomosity(value) => value,
            _ => panic!() 
        }
    }

    pub fn mandible_sharpness(&self) -> f32 {
        match self.genome[BeetleGeneIndex::MandibleSharpness as usize] {
            BeetleGene::MandibleSharpness(value) => value,
            _ => panic!() 
        }
    }

    pub fn set_gene_value(&mut self, index: BeetleGeneIndex, value: f32) {
        match index {
            BeetleGeneIndex::Size => {
                self.genome[BeetleGeneIndex::Size as usize] =
                    BeetleGene::Size(value);
            },
            BeetleGeneIndex::CarapaceDensity => {
                self.genome[BeetleGeneIndex::CarapaceDensity as usize] =
                    BeetleGene::CarapaceDensity(value);
            },
            BeetleGeneIndex::Strength => {
                self.genome[BeetleGeneIndex::Strength as usize] =
                    BeetleGene::Strength(value);
            },
            BeetleGeneIndex::Quickness => {
                self.genome[BeetleGeneIndex::Quickness as usize] =
                    BeetleGene::Quickness(value);
            },
            BeetleGeneIndex::Venomosity => {
                self.genome[BeetleGeneIndex::Venomosity as usize] =
                    BeetleGene::Venomosity(value);
            },
            BeetleGeneIndex::MandibleSharpness => {
                self.genome[BeetleGeneIndex::MandibleSharpness as usize] =
                    BeetleGene::MandibleSharpness(value);
            },
        }
    }
}



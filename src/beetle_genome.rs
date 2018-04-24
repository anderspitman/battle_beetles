use rand::{Rng, thread_rng};

#[derive(Serialize, Debug, Clone)]
pub struct BeetleGenome {
    genome: Vec<BeetleGene>,
}

// TODO: properly implement a Ratio type to use with these
// instead of f32
pub type Ratio = f32;

// should limit the values to 0.0-1.0
#[derive(Serialize, Debug, Clone)]
enum BeetleGene {
    CarapaceDensity(Ratio),
    Strength(Ratio),
    Quickness(Ratio),
    Venomosity(Ratio),
    MandibleSharpness(Ratio),
    BodyWidth(Ratio),
    BodyLength(Ratio),
}

pub enum BeetleGeneIndex {
    CarapaceDensity = 0,
    Strength = 1,
    Quickness = 2,
    Venomosity = 3,
    MandibleSharpness = 4,
    BodyWidth = 5,
    BodyLength = 6,
}

impl From<i32> for BeetleGeneIndex {
    fn from(val: i32) -> BeetleGeneIndex {
        match val {
            0 => BeetleGeneIndex::CarapaceDensity,
            1 => BeetleGeneIndex::Strength,
            2 => BeetleGeneIndex::Quickness,
            3 => BeetleGeneIndex::Venomosity,
            4 => BeetleGeneIndex::MandibleSharpness,
            5 => BeetleGeneIndex::BodyWidth,
            6 => BeetleGeneIndex::BodyLength,
            _ => panic!("Invalid gene index {}", val)
        }
    }
}

impl BeetleGenome {
    pub fn new() -> BeetleGenome {
        BeetleGenome {
            genome: vec![
                BeetleGene::CarapaceDensity(0.5),
                BeetleGene::Strength(0.5),
                BeetleGene::Quickness(0.5),
                BeetleGene::Venomosity(0.5),
                BeetleGene::MandibleSharpness(0.5),
                BeetleGene::BodyWidth(0.5),
                BeetleGene::BodyLength(0.5),
                // other gene ideas:
                // coordination (affects turning speed, etc)
                // mandible size
                // mandible strength
                // attack angle
                // more legs
            ],
        }
    }

    pub fn get_num_genes(&self) -> i32 {
        self.genome.len() as i32
    }

    pub fn get_random_gene_index(&self) -> BeetleGeneIndex {
        let num_genes = self.get_num_genes();
        let random_gene_index =
            thread_rng().gen_range::<i32>(0, num_genes);

        BeetleGeneIndex::from(random_gene_index)
    }

    pub fn set_random_genome(&mut self) {

        let mut rng = thread_rng();
        self.set_gene_value(BeetleGeneIndex::CarapaceDensity, rng.gen());
        self.set_gene_value(BeetleGeneIndex::Strength, rng.gen());
        self.set_gene_value(BeetleGeneIndex::Quickness, rng.gen());
        self.set_gene_value(BeetleGeneIndex::Venomosity, rng.gen());
        self.set_gene_value(BeetleGeneIndex::MandibleSharpness, rng.gen());
        self.set_gene_value(BeetleGeneIndex::BodyWidth, rng.gen());
        self.set_gene_value(BeetleGeneIndex::BodyLength, rng.gen());
    }

    pub fn get_gene(
            &self, gene_index: BeetleGeneIndex) -> Ratio {

        match self.genome[gene_index as usize] {
            BeetleGene::CarapaceDensity(value) => value,
            BeetleGene::Strength(value) => value,
            BeetleGene::Quickness(value) => value,
            BeetleGene::Venomosity(value) => value,
            BeetleGene::MandibleSharpness(value) => value,
            BeetleGene::BodyWidth(value) => value,
            BeetleGene::BodyLength(value) => value,
        }
    }

    pub fn set_gene_value(
            &mut self, index: BeetleGeneIndex, value: f32) {

        match index {
            BeetleGeneIndex::CarapaceDensity => {
                self.genome[BeetleGeneIndex::CarapaceDensity
                        as usize] =
                    BeetleGene::CarapaceDensity(value);
            },
            BeetleGeneIndex::Strength => {
                self.genome[BeetleGeneIndex::Strength
                        as usize] =
                    BeetleGene::Strength(value);
            },
            BeetleGeneIndex::Quickness => {
                self.genome[BeetleGeneIndex::Quickness
                        as usize] =
                    BeetleGene::Quickness(value);
            },
            BeetleGeneIndex::Venomosity => {
                self.genome[BeetleGeneIndex::Venomosity
                        as usize] =
                    BeetleGene::Venomosity(value);
            },
            BeetleGeneIndex::MandibleSharpness => {
                self.genome[BeetleGeneIndex::MandibleSharpness
                        as usize] =
                    BeetleGene::MandibleSharpness(value);
            },
            BeetleGeneIndex::BodyWidth => {
                self.genome[BeetleGeneIndex::BodyWidth
                        as usize] =
                    BeetleGene::BodyWidth(value);
            },
            BeetleGeneIndex::BodyLength => {
                self.genome[BeetleGeneIndex::BodyLength
                        as usize] =
                    BeetleGene::BodyLength(value);
            },
        }
    }
}



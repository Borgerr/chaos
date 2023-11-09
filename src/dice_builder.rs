use crate::dice;
pub struct DiceBuilder {
    pub dice_sides: u32,
    pub dice_name: String,
}

impl DiceBuilder {
    pub fn sides(&mut self, sides: u32) -> &mut Self {
        self.dice_sides = sides;
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.dice_name = name;
        self
    }

    pub fn build(&self) -> dice::Dice {
        dice::Dice {
            previous_result: 0,
            sides: self.dice_sides,
            name: self.dice_name.clone(),
        }
    }
}

use crate::dice_builder;
pub struct DiceBuilderBuilder {
    pub dice_builder_sides: u32,
    pub dice_builder_name: String,
}

impl DiceBuilderBuilder {
    pub fn sides(mut self, dice_sides: u32) -> Self {
        self.dice_builder_sides = dice_sides;
        self
    }

    pub fn name(mut self, dice_name: String) -> Self {
        self.dice_builder_name = dice_name;
        self
    }

    pub fn build(self) -> dice_builder::DiceBuilder {
        dice_builder::DiceBuilder {
            dice_sides: self.dice_builder_sides,
            dice_name: self.dice_builder_name.clone(),
        }
    }
}

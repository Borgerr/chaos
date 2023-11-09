use crate::dice_builder_builder;

pub struct DiceBuilderBuilderFactory {}

impl DiceBuilderBuilderFactory {
    pub fn dice_builder_builder_factory(&self) -> dice_builder_builder::DiceBuilderBuilder {
        dice_builder_builder::DiceBuilderBuilder {
            dice_builder_sides: 0,
            dice_builder_name: "Default".parse().unwrap(),
        }
    }
}

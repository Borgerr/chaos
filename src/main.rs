use crate::dice_builder_builder_factory::DiceBuilderBuilderFactory;

mod dice;
mod dice_builder;
mod dice_builder_builder;
mod dice_builder_builder_factory;

fn main() {
    let dice_builder_builder_factory = DiceBuilderBuilderFactory {};
    let mut dice_builder_builder = dice_builder_builder_factory.dice_builder_builder_factory();
    let mut dice_builder = dice_builder_builder
        .name("Dicey".to_string())
        .sides(6)
        .build();
    let mut dice = dice_builder.name("D20".to_string()).sides(20).build();

    dice.roll();
}

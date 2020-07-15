use autovivification::{autoviv, iviv, stringify, viv, Any, BorrowMut, Debug, HashMap};

fn main() {
    let mut food = autoviv();

    viv!(food.cold.dessert.quantity, 100);
    println!("{:?}", viv!(food.cold.dessert.quantity));

    viv!(food.cold.dessert.pudding, "chocolate");
    println!("{:?}", viv!(food.cold.dessert.pudding));

    viv!(food.cold.dessert.icecream.grades, [1, 2, 3, 4, 5]);
    println!("{:?}", viv!(food.cold.dessert.icecream.grades));

    viv!(
        food.export.countries,
        vec!["India", "Denmark", "Sweden", "Japan"]
    );
    println!("{:?}", viv!(food.export.countries));
}

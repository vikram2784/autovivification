use autovivification::{autoviv, iviv, stringify, viv, Any, BorrowMut, Debug, HashMap};

fn main() {
    let mut food = autoviv();

    viv!(food, "tasty");
    println!("{:?}", viv!(food));

    viv!(food.cold.dessert.icecream, "chocobar");
    println!("{:?}", viv!(food.cold.dessert.icecream));

    println!("{:?}", viv!(food)); // {..} prints keys in the object

    viv!(food.cold.dessert.icecream, "vanillacup"); //overwrite
    println!("{:?}", viv!(food.cold.dessert.icecream));

    viv!(food.cold.dessert.custard, "fruit");
    println!("{:?}", viv!(food.cold.dessert.custard));

    println!("{:?}", viv!(food.cold)); // {..} prints keys in the object
    println!("{:?}", viv!(food.cold.dessert)); // {..} prints keys in the object
    println!("{:?}", viv!(food.cold.dessert.pudding)); //None, as it is not set
}

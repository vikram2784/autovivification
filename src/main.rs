use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Debug;
use std::stringify;


macro_rules! viv {
    ($last:ident, $b:expr, $e:expr) => {{
        if $b {
            let mut __value:HashMap<&str, Box<dyn Debug>> = HashMap::new();
            __value.insert("__value", Box::new($e));
            let __value_box = Box::new(__value);
            $last.clear();
            $last.insert("__value", __value_box as Box<dyn Any>);
        }

        if $last.len() > 1 {
            $last.remove("__value");
            Some($last as &dyn Debug)
        } else {
            if let Some(val) = $last.get("__value") {
                Some(&**val
                    .downcast_ref::<HashMap<&str, Box<dyn Debug>>>()
                    .unwrap()
                    .get("__value")
                    .unwrap()) 
            } else if $last.len() > 0 {
                Some($last as &dyn Debug)
            } else {
                None
            }
        }
    }};

    ($outer:ident.$inner:ident$(.$inner1:ident)*, $b:expr, $e:expr) => {{
        if !$outer.contains_key(stringify!($inner)) {
            $outer.insert(stringify!($inner), autoviv());
        }

        let $inner = $outer
                        .get_mut(stringify!($inner))
                        .unwrap()
                        .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                        .unwrap();
        viv! ($inner$(.$inner1)*, $b, $e)
    }};

    ($outer:ident.$inner:ident$(.$inner1:ident)*, $e:expr) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();
        viv!(__root.$inner$(.$inner1)*, true, $e)
    }};

    ($outer:ident.$inner:ident$(.$inner1:ident)*) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        viv!(__root.$inner$(.$inner1)*, false, 0)
    }};

    ($outer:ident, $e:expr) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        viv!(__root, true, $e)
    }};

    ($outer:ident) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        viv!(__root, false, 0)
    }};
}

fn autoviv() -> Box<dyn Any> {
    Box::new(HashMap::<&'static str, Box<dyn Any>>::new())
}

fn main() {
    let mut food = autoviv();

    viv!(food, "tasty");
    println!("{:?}", viv!(food));

    viv!(food.cold.dessert.icecream, "chocobar");
    println!("{:?}", viv!(food.cold.dessert.icecream));

    println!("{:?}", viv!(food));  // {..} prints keys in the object

    viv!(food.cold.dessert.icecream, "vanillacup"); //overwrite
    println!("{:?}", viv!(food.cold.dessert.icecream));

    viv!(food.cold.dessert.custard, "fruit");
    println!("{:?}", viv!(food.cold.dessert.custard));

    println!("{:?}", viv!(food.cold)); // {..} prints keys in the object
    println!("{:?}", viv!(food.cold.dessert)); // {..} prints keys in the object
    println!("{:?}", viv!(food.cold.dessert.pudding)); //None, as it is not set

    /*
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
    */
}

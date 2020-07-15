#![allow(unused_macros, unused_imports)]

pub use std::any::Any;
pub use std::collections::HashMap;
pub use std::borrow::BorrowMut;
pub use std::fmt::Debug;
pub use std::stringify;

#[macro_export]
macro_rules! iviv {
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
        iviv! ($inner$(.$inner1)*, $b, $e)
    }};

}

#[macro_export]
macro_rules! viv {
    ($outer:ident.$inner:ident$(.$inner1:ident)*, $e:expr) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();
        iviv!(__root.$inner$(.$inner1)*, true, $e)
    }};

    ($outer:ident.$inner:ident$(.$inner1:ident)*) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        iviv!(__root.$inner$(.$inner1)*, false, 0)
    }};

    ($outer:ident, $e:expr) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        iviv!(__root, true, $e)
    }};

    ($outer:ident) => {{
        let __r:&mut dyn Any = $outer.borrow_mut();
        let __root = __r
                    .downcast_mut::<HashMap<&str, Box<dyn Any>>>()
                    .unwrap();

        iviv!(__root, false, 0)
    }};
}

pub fn autoviv() -> Box<dyn Any> {
    Box::new(HashMap::<&'static str, Box<dyn Any>>::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer() {
        let mut food = autoviv();
        viv!(food, "tasty");
        viv!(food).unwrap();
    }

    #[test]
    fn test_nested() {
        let mut food = autoviv();
        viv!(food.cold.dessert.count, 11);
        viv!(food.cold.dessert.count).unwrap();
    }

    #[test]
    fn test_nested2() {
        let mut food = autoviv();
        viv!(food.cold.dessert.grades, vec![1, 2, 3]);
        viv!(food.cold.dessert.grades).unwrap();
    }

    #[test]
    fn test_outer_obj_key() {
        let mut food = autoviv();
        viv!(food.cold.dessert.icecream, "chocobar");
        viv!(food.cold).unwrap();
    }

    #[test]
    fn test_middle_obj_key() {
        let mut food = autoviv();
        viv!(food.cold.dessert.icecream, "chocobar");
        viv!(food.cold.dessert).unwrap();
    }

    #[test]
    fn test_outer_obj_key_absence() {
        let mut food = autoviv();
        viv!(food.cold.dessert.icecream, "chocobar");
        assert!(if let Some(_) = viv!(food.hot) {
            false
        } else {
            true
        });
    }

    #[test]
    fn test_middle_obj_key_absence() {
        let mut food = autoviv();
        viv!(food.cold.dessert.icecream, "chocobar");
        assert!(if let Some(_) = viv!(food.hot.cold) {
            false
        } else {
            true
        });
    }

    #[test]
    fn test_middle_obj_key_absence2() {
        let mut food = autoviv();
        assert!(if let Some(_) = viv!(food.hot.noodles) {
            false
        } else {
            true
        });
    }

    #[test]
    fn test_obj_key_presence_after_read() {
        let mut food = autoviv();
        viv!(food.hot.noodles);
        viv!(food.hot).unwrap();
    }
}

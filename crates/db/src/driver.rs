// driver.rs

pub trait Driver {
    fn insert(&self) -> i64;
    fn update(&self) -> i64;
    fn get(&self);
    fn delete(&self) -> bool;
}

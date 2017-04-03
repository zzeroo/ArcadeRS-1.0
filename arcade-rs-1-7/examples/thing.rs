pub trait Thing {
    fn do_something_interesting(&self) -> i32;
}

pub struct A;
impl Thing for A {
    fn do_something_interesting(&self) -> i32 {
        1
    }
}

pub struct B;
impl Thing for B {
    fn do_something_interesting(&self) -> i32 {
        2
    }
}

pub fn do_something_to_everyone(everyone: &Vec<Box<Thing>>) -> Vec<i32> {
    everyone.iter().map(|thing| thing.do_something_interesting()).collect()
}


fn main() {
    let mut everyone: Vec<Box<Thing>> = vec![];
    everyone.push(Box::new(A));
    everyone.push(Box::new(B));
    everyone.push(Box::new(B));
    everyone.push(Box::new(A));
    let res = do_something_to_everyone(&everyone);
    println!("{:#?}", res);
}

mod complex;
mod simple;
mod time_complexity;

pub trait List {
    fn insert(&mut self, value: i32) -> &mut dyn List;
    fn delete(&mut self, value: i32) -> &mut dyn List;
}

fn main() {
    let complex_time =
        crate::time_complexity::TimeComplexity::new(Box::new(crate::complex::LinkedList::new()))
            .compute();

    println!("Complex time: {}", complex_time);
}

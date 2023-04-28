use crate::List;

pub struct TimeComplexity {
    tree: Box<dyn List>,
}

impl TimeComplexity {
    pub fn new(tree: Box<dyn List>) -> Self {
        Self { tree }
    }
    pub fn compute(&mut self) -> f64 {
        let largest = 10000;
        let mut start = std::time::Instant::now();
        for i in 0..largest {
            self.tree.insert(i);
        }
        let mut end = std::time::Instant::now();
        let insert_time = end.duration_since(start).as_secs_f64();

        start = std::time::Instant::now();
        for i in 0..largest {
            self.tree.delete(i);
        }
        end = std::time::Instant::now();
        let delete_time = end.duration_since(start).as_secs_f64();

        insert_time / delete_time
    }
}

pub struct Entitys {
    next_id: usize,
}

impl Entitys {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn new_id(&mut self) -> usize {
        let id = self.next_id;

        self.next_id += 1;

        return id;
    }
}

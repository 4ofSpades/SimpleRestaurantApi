#[derive(Debug)]
struct Item {
    id: u16,
    name: String,
    duration_minutes: u16,
}

impl Item {
    fn get_duration_in_millis(&self) -> u32 {
        self.duration_minutes * 60000
    } 
}
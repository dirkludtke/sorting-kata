/// counter of the comparison operations which pretends that one comparion takes 1 seconds.
#[derive(Debug)]
pub struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {

    pub fn new_with_limit(limit: u32) -> Self {
        Counter { count: 0, limit }
    }

    pub fn new() -> Self {
        Self::new_with_limit(3600)
    }

    pub fn increment(&mut self) {
        self.count += 1;
        if self.count >= self.limit {
            panic!(
                "Abort at count {}. That corresponds to {}!",
                self.count,
                self.to_string()
            );
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

impl ToString for Counter {
    fn to_string(&self) -> String {
        let mut count = self.count;
        if count < 60 {
            return format!("{} second{}", count, if count != 1 { "s" } else { "" });
        }
        count = (count + 30) / 60;
        if count < 60 {
            return format!("{} minute{}", count, if count != 1 { "s" } else { "" });
        }
        count = (count + 30) / 60;
        if count < 24 {
            return format!("{} hour{}", count, if count != 1 { "s" } else { "" });
        }
        count = (count + 12) / 24;
        if count < 366 {
            return format!("{} day{}", count, if count != 1 { "s" } else { "" });
        }
        count = (count + 183) / 366;
        format!("{} year{}", count, if count != 1 { "s" } else { "" })
    }
}

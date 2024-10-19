#!allow[unused_imports, dead_code]

#[derive(Clone)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
}


#[derive(Clone)]
pub struct Group {
    num: u8,
    unit: TimeUnit,
}


#[derive(Clone)]
pub struct Recent {
    num: u8,
    unit: TimeUnit,
}

#[derive(Clone)]
pub struct Period {
    start: String,
    end: String,
}
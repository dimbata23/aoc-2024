use std::collections::HashMap;
use std::io;

type DayFunction = fn() -> io::Result<()>;
type DayMap = HashMap<&'static str, DayFunction>;

pub fn register_days() -> DayMap {
    let mut days: DayMap = HashMap::new();
    use day01;
    days.insert("day01", day01::run);

    days
}

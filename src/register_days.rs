use std::collections::HashMap;
use std::io;

type DayFunction = fn() -> io::Result<()>;
type DayMap = HashMap<&'static str, DayFunction>;

pub fn register_days() -> DayMap {
    let mut days: DayMap = HashMap::new();
    use day01;
    days.insert("day01", day01::run);
    use day04;
    days.insert("day04", day04::run);
    use day03;
    days.insert("day03", day03::run);
    use day02;
    days.insert("day02", day02::run);

    days
}

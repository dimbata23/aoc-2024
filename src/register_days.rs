use std::collections::HashMap;
use std::io;

type DayFunction = fn() -> io::Result<()>;
type DayMap = HashMap<&'static str, DayFunction>;

pub fn register_days() -> DayMap {
    let mut days: DayMap = HashMap::new();
    use day07;
    days.insert("day07", day07::run);
    use day09;
    days.insert("day09", day09::run);
    use day08;
    days.insert("day08", day08::run);
    use day06;
    days.insert("day06", day06::run);
    use day01;
    days.insert("day01", day01::run);
    use day12;
    days.insert("day12", day12::run);
    use day15;
    days.insert("day15", day15::run);
    use day14;
    days.insert("day14", day14::run);
    use day13;
    days.insert("day13", day13::run);
    use day04;
    days.insert("day04", day04::run);
    use day03;
    days.insert("day03", day03::run);
    use day02;
    days.insert("day02", day02::run);
    use day05;
    days.insert("day05", day05::run);
    use day18;
    days.insert("day18", day18::run);
    use day16;
    days.insert("day16", day16::run);
    use day11;
    days.insert("day11", day11::run);
    use day10;
    days.insert("day10", day10::run);
    use day17;
    days.insert("day17", day17::run);
    use day19;
    days.insert("day19", day19::run);

    days
}

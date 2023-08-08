use itertools::Itertools;

type Range = (usize, usize);

fn parse_range(range: &str) -> Range {
    range
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple::<Range>()
        .unwrap()
}

fn parse_line(line: &str) -> (Range, Range) {
    line.split(",").map(parse_range).collect_tuple().unwrap()
}

fn fully_contained_in_range((min_a, max_a): &Range, (min_b, max_b): &Range) -> bool {
    min_a <= min_b && max_a >= max_b
}

fn contained_below((min_a, max_a): &Range, (min_b, _max_b): &Range) -> bool {
    min_a <= min_b && max_a >= min_b
}

fn contained_above((min_a, max_a): &Range, (_min_b, max_b): &Range) -> bool {
    max_a >= max_b && min_a >= max_b
}

fn contained_in_range(range_a: &Range, range_b: &Range) -> bool {
    let (min_a, max_a) = range_a;
    let (min_b, max_b) = range_b;
    let below = if min_a < min_b {
        contained_below(range_a, range_b)
    } else {
        contained_below(range_b, range_a)
    };
    let above = if max_a < max_b {
        contained_above(range_a, range_b)
    } else {
        contained_above(range_b, range_a)
    };

    below || above
}

fn main() {
    let contents = std::fs::read_to_string("data/test.txt").unwrap();

    let input = contents.split("\n").into_iter().map(parse_line);

    let problem1 = input
        .clone()
        .filter(|(range_a, range_b)| {
            fully_contained_in_range(range_a, range_b) || fully_contained_in_range(range_b, range_a)
        })
        .count();

    println!("{}", problem1);

    let problem2 = input
        .filter(|(range_a, range_b)| contained_in_range(range_a, range_b))
        .count();

    println!("{}", problem2);
}

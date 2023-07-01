#[derive(Debug, Eq, PartialEq)]
enum Interval {
    Zero,
    One(usize),
    Two(usize, usize),
}

impl Interval {
    fn plus(&self, next: usize) -> Interval {
        use Interval::*;
        match self {
            Zero => One(next),
            One(a) => match next.cmp(a) {
                std::cmp::Ordering::Less => Two(next, *a),
                std::cmp::Ordering::Equal => One(*a),
                std::cmp::Ordering::Greater => Two(*a, next),
            }
            Two(a, b) => match next.cmp(a) {
                std::cmp::Ordering::Less => Two(next, *b),
                std::cmp::Ordering::Equal => Two(*a, *b),
                std::cmp::Ordering::Greater => match next.cmp(b) {
                    std::cmp::Ordering::Less => Two(*a, *b),
                    std::cmp::Ordering::Equal => Two(*a, *b),
                    std::cmp::Ordering::Greater => Two(*a, next),
                }
            }
        }
    }

    fn min(&self) -> Option<usize> {
        use Interval::*;
        match self {
            Zero => None,
            One(a) => Some(*a),
            Two(a, _) => Some(*a),
        }
    }

    fn max(&self) -> Option<usize> {
        use Interval::*;
        match self {
            Zero => None,
            One(a) => Some(*a),
            Two(_, b) => Some(*b),
        }
    }
}

#[cfg(test)]
mod interval_tests {
    use super::Interval::*;

    #[test]
    fn build_tables() {
        let mut intervals = vec![Zero];
        for i in 0..7 {
            intervals.push(One(i));
        }

        for i in 0..7 {
            for j in (i+1)..7 {
                intervals.push(Two(i, j));
            }
        }
        
        let mut promotion_table = [[0; 7]; 29];

        for (pos, interval) in intervals.iter().enumerate() {
            for i in 0..7 {
                let sum = interval.plus(i);
                println!("{:?} + {} = {:?}", interval, i, sum);
                let sum_pos = intervals.iter().position(|x| *x == sum).unwrap();
                promotion_table[pos][i] = sum_pos;
            }
        }
        println!("{promotion_table:?}");
    }
}

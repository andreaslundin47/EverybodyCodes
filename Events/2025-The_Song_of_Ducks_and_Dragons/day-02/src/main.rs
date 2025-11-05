use itertools::Itertools;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn parse_a(s: &str) -> Complex {
    let s = s
        .trim()
        .strip_prefix("A=[")
        .unwrap()
        .strip_suffix("]")
        .unwrap();
    let (re, im) = s.split_once(',').unwrap();

    Complex {
        re: re.parse::<i64>().unwrap(),
        im: im.parse::<i64>().unwrap(),
    }
}

fn part_one() {
    let a = parse_a(include_str!("../input1.txt"));
    let d = Complex { re: 10, im: 10 };
    let mut r = Complex { re: 0, im: 0 };

    for _ in 0..3 {
        r = r.cycle(&a, &d);
    }

    println!("Part 1. [{},{}]", r.re, r.im);
}

fn part_two() {
    let a_base = parse_a(include_str!("../input2.txt"));
    let d = Complex {
        re: 100_000,
        im: 100_000,
    };

    let mut engraved = 0;

    for (x, y) in (0..101).cartesian_product(0..101) {
        let a = Complex {
            re: a_base.re + 10 * x,
            im: a_base.im + 10 * y,
        };

        if Complex::valid_hundred(&a, &d) {
            engraved += 1;
        }
    }

    println!("Part 2. {engraved}");
}

fn part_three() {
    let a_base = parse_a(include_str!("../input3.txt"));
    let d = Complex {
        re: 100_000,
        im: 100_000,
    };

    let mut engraved = 0;

    for (x, y) in (0..1001).cartesian_product(0..1001) {
        let a = Complex {
            re: a_base.re + x,
            im: a_base.im + y,
        };

        if Complex::valid_hundred(&a, &d) {
            engraved += 1;
        }
    }

    println!("Part 3. {engraved}");
}

#[derive(Clone)]
struct Complex {
    re: i64,
    im: i64,
}

impl Complex {
    fn add(mut self, other: &Self) -> Self {
        let re = self.re + other.re;
        let im = self.im + other.im;
        self.re = re;
        self.im = im;
        return self;
    }

    fn mul(mut self, other: &Self) -> Self {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + other.im * self.re;
        self.re = re;
        self.im = im;
        return self;
    }

    fn div(mut self, other: &Self) -> Self {
        let re = self.re / other.re;
        let im = self.im / other.im;
        self.re = re;
        self.im = im;
        return self;
    }

    fn cycle(mut self, a: &Self, d: &Self) -> Self {
        let rc = self.clone();
        self = self.mul(&rc);
        self = self.div(&d);
        self = self.add(&a);

        self
    }

    fn valid_hundred(a: &Self, d: &Self) -> bool {
        let mut r = Complex { re: 0, im: 0 };

        for _ in 0..100 {
            r = r.cycle(a, d);

            if r.is_excessive() {
                return false;
            }
        }

        true
    }

    fn is_excessive(&self) -> bool {
        self.re.abs() >= 1_000_000 || self.im.abs() >= 1_000_000
    }
}

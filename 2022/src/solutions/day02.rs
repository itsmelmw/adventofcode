pub fn solve(input: String) -> (String, String) {
    let mut a = 0;
    let mut b = 0;

    for line in input.split("\n") {
        let (a_score, b_score) = match line.as_bytes() {
            [b'A', .., b'X'] => (1 + 3, 3 + 0),
            [b'A', .., b'Y'] => (2 + 6, 1 + 3),
            [b'A', .., b'Z'] => (3 + 0, 2 + 6),
            [b'B', .., b'X'] => (1 + 0, 1 + 0),
            [b'B', .., b'Y'] => (2 + 3, 2 + 3),
            [b'B', .., b'Z'] => (3 + 6, 3 + 6),
            [b'C', .., b'X'] => (1 + 6, 2 + 0),
            [b'C', .., b'Y'] => (2 + 0, 3 + 3),
            [b'C', .., b'Z'] => (3 + 3, 1 + 6),
            _ => panic!("Invalid line in input"),
        };
        a += a_score;
        b += b_score;
    }

    return (a.to_string(), b.to_string());
}

struct Top3 {
    list: [usize; 3],
    min_ind: usize,
}

impl Top3 {
    fn init() -> Self {
        return Top3 {
            list: [0, 0, 0],
            min_ind: 0,
        };
    }

    fn top(&self) -> usize {
        return *self.list.iter().max().unwrap();
    }

    fn sum(&self) -> usize {
        return self.list.iter().sum();
    }

    fn update_min(&mut self) {
        self.min_ind = 0;
        for i in [1, 2] {
            if self.list[i] < self.list[self.min_ind] {
                self.min_ind = i;
            }
        }
    }

    fn update(&mut self, new: usize) {
        if new > self.list[self.min_ind] {
            self.list[self.min_ind] = new;
            self.update_min();
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let totals = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|cal| cal.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<Vec<usize>>();

    let mut top3 = Top3::init();

    for &total in totals.iter() {
        top3.update(total);
    }

    return (top3.top().to_string(), top3.sum().to_string());
}

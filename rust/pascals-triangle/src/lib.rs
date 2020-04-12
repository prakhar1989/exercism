pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle {
                triangle: Vec::new(),
            };
        }

        let mut triangle: Vec<Vec<u32>> = vec![vec![1]];
        let row_count = row_count as usize;

        for row_index in 1..row_count {
            triangle.push(
                (0..row_index + 1)
                    .map(|j| {
                        if j == 0 || j == row_index {
                            return 1;
                        }
                        let prev_row = &triangle[row_index - 1];
                        prev_row[j - 1] + prev_row[j]
                    })
                    .collect::<Vec<u32>>(),
            );
        }

        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.iter().map(|v| v.clone()).collect()
    }
}

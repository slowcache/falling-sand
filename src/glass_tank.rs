pub struct Tank {
    pub grains: Vec<Vec<char>>,
    step: usize
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    NEITHER,
}

impl Tank {
    pub fn new(width: usize, height: usize, step: usize) -> Tank {
        assert!(width > 0, "Width must be greater than 0");
        assert!(height > 0, "Height must be greater than 0");
        Self {
            grains: vec![vec!['-'; width]; height],
            step
        }
    }

    pub fn from_grains(v: Vec<String>) -> Tank {
        assert!(v.len() > 0, "from_grains: Height must be greater than 0!");
        assert!(v[0].len() > 0, "from_grains: Width must be greater than 0!");
        Self {
            grains: {
                let mut outer: Vec<Vec<char>> = vec![];

                for line in v.iter() {
                    let mut inner: Vec<char> = vec![];
                    for char in line.chars() {
                        inner.push(char);
                    }
                    outer.push(inner);
                }
            
                outer
            },
            step: 0
        }
    }

    pub fn advance_frame(&mut self) {
        for row in (0..self.grains.len()).rev() {
            let mut direction_moved = Direction::NEITHER;
            for column in 0..self.grains[row].len() {
                if self.grains[row][column] == '-'  || row == self.grains.len() - 1 {
                    direction_moved = Direction::NEITHER;
                    continue;
                }

                if direction_moved == Direction::RIGHT {
                    // This is because if a grain moves to the right we will try to reprocess it
                    // This prevents a grain from being processed twice in 1 frame
                    direction_moved = Direction::NEITHER;
                    continue;
                }

                let fell = self.fall_down(row, column);

                if !fell {
                    direction_moved = self.move_horizontally(row, column);
                }
            }
        }
    }

    fn fall_down(&mut self, row: usize, column: usize) -> bool {
        if self.grains[row + 1][column] != '-' {
            return false;
        }

        self.grains[row + 1][column] = self.grains[row][column];
        self.grains[row][column] = '-';
        return true;
    }

    fn move_horizontally(&mut self, row: usize, column: usize) -> Direction {
        let direction: Direction = self.figure_out_direction_to_move(row, column);

        match direction {
            Direction::LEFT => {
                self.grains[row][column - 1] = self.grains[row][column];
                self.grains[row][column] = '-';
            },
            Direction::RIGHT => {
                self.grains[row][column + 1] = self.grains[row][column];
                self.grains[row][column] = '-';
            },
            Direction::NEITHER => ()
        };

        direction
    }

    fn figure_out_direction_to_move(&self, row: usize, column: usize) -> Direction {
        let can_move_left = column != 0 && self.grains[row][column - 1] == '-';
        let can_move_right = column + 1 < self.grains[row].len() && self.grains[row][column + 1] == '-';

        if !can_move_left && !can_move_right {
            return Direction::NEITHER;
        }

        let left_drop: usize = match can_move_left {
            true => (self.grains.len() - row) - self.get_count_of_grains_in_column(column - 1),
            false => 0
        };

        let right_drop: usize = match can_move_right {
            true => (self.grains.len() - row) - self.get_count_of_grains_in_column(column + 1),
            false => 0
        };

        // Prefer left movement
        if left_drop != 0 && left_drop >= right_drop && left_drop - 1 > self.step {
            return Direction::LEFT;
        }

        if right_drop != 0 && right_drop > left_drop && right_drop - 1 > self.step {
            return Direction::RIGHT;
        }

        return Direction::NEITHER;
    }

    fn get_count_of_grains_in_column(&self, column: usize) -> usize {
        let mut count = 0;
        for i in (0..self.grains.len()).rev() {
            if self.grains[i][column] != '-' {
                count = count + 1;
            } else {
                break;
            }
        }
        return count;
    }

    pub fn drop_sand_in_column(&mut self, column: usize, grain: char) {
        self.drop_sand(0, column, grain, 1);
    }

    pub fn drop_sand(&mut self, row: usize, column: usize, grain: char, area: usize) {
        assert!(column < self.grains[0].len(), "Cannot drop sand past a column larger than the width of the tank!");

        for i in 0..area {
            if row + i >= self.grains.len() {
                break;
            }

            for j in 0..area {
                if column + j >= self.grains[i].len() {
                    break;
                }
                    
                if self.grains[row + i][column + j] == '-' {
                    self.grains[row + i][column + j] = grain;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("");
        for v in self.grains.iter() {
            for c in v.iter() {
                s.push(*c);
            }
            s.push('\n');
        }
        s
    }

    pub fn equals(&self, other: &Tank) -> bool {
        if self.grains.len() != other.grains.len() {
            return false;
        }

        if self.grains[0].len() != other.grains[0].len() {
            return false;
        }

        for i in 0..self.grains.len() {
            for j in 0..self.grains[i].len() {
                if self.grains[i][j] != other.grains[i][j] {
                    return false;
                }
            }
        }

        return true;
    }
}

/////////////// TEST CODE //////////////////////////
/////////////// TEST CODE //////////////////////////
/////////////// TEST CODE //////////////////////////
/////////////// TEST CODE //////////////////////////
/////////////// TEST CODE //////////////////////////
/////////////// TEST CODE //////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_column_init() {
        let t = Tank::new(1, 3, 0);
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "-".to_string(),
            "-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_drop_sand() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        let expected = Tank::from_grains(vec!(
            "g".to_string(),
            "-".to_string(),
            "-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_advance_frame1() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "g".to_string(),
            "-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_advance_frame2() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "-".to_string(),
            "g".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_advance_frame3() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "-".to_string(),
            "g".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_two_grain() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        let expected = Tank::from_grains(vec!(
            "g".to_string(),
            "g".to_string(),
            "-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_two_grain_advance_frame1() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "g".to_string(),
            "g".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_one_column_two_grain_advance_frame2() {
        let mut t = Tank::new(1, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-".to_string(),
            "g".to_string(),
            "g".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_two_grain_step0_advance_frame1() {
        let mut t = Tank::new(2, 2, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "-g".to_string(),
            "g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_two_grain_step0_advance_frame2() {
        let mut t = Tank::new(2, 2, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "--".to_string(),
            "gg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_three_grain_step0_setup() {
        let mut t = Tank::new(2, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "--".to_string(),
            "gg".to_string(),
            "g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_three_grain_step0_advance_frame1() {
        let mut t = Tank::new(2, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "--".to_string(),
            "g-".to_string(),
            "gg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_two_grain_step1() {
        let mut t = Tank::new(2, 2, 1);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        let expected = Tank::from_grains(vec!(
            "g-".to_string(),
            "g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_two_column_two_grain_step1_advance_frame1() {
        let mut t = Tank::new(2, 2, 1);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "g-".to_string(),
            "g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_setup() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "---".to_string(),
            "-g-".to_string(),
            "-g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_advance_frame1() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "---".to_string(),
            "g--".to_string(),
            "-g-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_advance_frame2() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "---".to_string(),
            "---".to_string(),
            "gg-".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_drop_another_in_middle() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();
        let expected = Tank::from_grains(vec!(
            "---".to_string(),
            "---".to_string(),
            "ggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_drop_6_on_left_step0() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();

        let expected = Tank::from_grains(vec!(
            "g--".to_string(),
            "gg-".to_string(),
            "ggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_drop_6_on_middle_step0() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.drop_sand_in_column(1, 'g');
        t.advance_frame();
        t.advance_frame();

        let expected = Tank::from_grains(vec!(
            "---".to_string(),
            "ggg".to_string(),
            "ggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_three_column_drop_6_on_right_step0() {
        let mut t = Tank::new(3, 3, 0);
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();

        let expected = Tank::from_grains(vec!(
            "--g".to_string(),
            "-gg".to_string(),
            "ggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_five_column_drop_15_on_left_step0() {
        let mut t = Tank::new(5, 5, 0);
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(0, 'g');
        t.advance_frame();
        t.advance_frame();

        let expected = Tank::from_grains(vec!(
            "g----".to_string(),
            "gg---".to_string(),
            "ggg--".to_string(),
            "gggg-".to_string(),
            "ggggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_five_column_drop_15_on_middle_step0() {
        let mut t = Tank::new(5, 5, 0);
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.drop_sand_in_column(2, 'g');
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();
        t.advance_frame();

        let expected = Tank::from_grains(vec!(
            "-----".to_string(),
            "--g--".to_string(),
            "gggg-".to_string(),
            "ggggg".to_string(),
            "ggggg".to_string()
        ));
        assert!(t.equals(&expected), "expected \n{} \n actual\n{}", expected.to_string(), t.to_string());
    }

    #[test]
    fn test_straight_line() {
        let t = Tank::from_grains(vec!(
            "--------G----------------------".to_string(),
            "--------G----------------------".to_string(),
            "-----------G------------------".to_string(),
            "--------G----------------------".to_string(),
            "--------G----------------------".to_string(),
            "--------G----------------------".to_string()
        ));

        assert!(t.get_count_of_grains_in_column(8) == 3, "get_count_of_grains_in_column is incorrect, expected: {} actual: {}", 3, t.get_count_of_grains_in_column(8));
        assert!(t.get_count_of_grains_in_column(11) == 0, "get_count_of_grains_in_column is incorrect, expected: {} actual: {}", 0, t.get_count_of_grains_in_column(11));
    }
}


pub struct TicTacToe {
    pub width: u32,
    pub height: u32,
    pub board: Vec<u8>,
}

impl TicTacToe {
    pub fn new(h: u32, w: u32) -> TicTacToe {
        return TicTacToe {
            width: w,
            height: h,
            board: vec![0; (w * h) as usize],
        };
    }
    pub fn show_board(&self) {
        for i in 0..=self.height - 1 {
            print!("{} \t:", i);
            for x in 0..=self.width - 1 {
                let index = ((i * self.width) + x) as usize;
                //print!("{} {} ||  ", self.board[index], index);
                print!("{} ||  ", self.board[index]);
            }
            println!();
        }
        println!();
    }
    pub fn set_space(&mut self, width: u32, height: u32, state: u8) {
        let index: usize = self.coord_to_index(width, height);
        self.board[index] = state;
    }
    fn get_state(&self, width: u32, height: u32) -> u8 {
        let index: usize = self.coord_to_index(width, height);
        return self.board[index];
    }
    fn coord_to_index(&self, width: u32, height: u32) -> usize {
        assert!(self.is_oob(width, height));
        return ((height * self.width) + width) as usize;
    }
    fn vertical_check(&self) -> u8 {
        let target_length: u8 = 3;
        let mut counter: u8 = 0;
        let mut current_state = 0;
        for i in 0..=self.height - 1 {
            for j in 0..=self.width - 1 {
                let state = self.get_state(j, i);
                if state == 0 {
                    continue;
                };
                if current_state == state {
                    counter += 1
                } else {
                    counter = 1;
                    current_state = state;
                }
                if counter == target_length {
                    return current_state;
                }
            }
            counter = 0;
        }
        return 0;
    }
    fn horizontal_check(&self) -> u8 {
        let target_length: u8 = 3;
        let mut counter: u8 = 0;
        let mut current_state = 0;
        for i in 0..=self.width - 1 {
            for j in 0..=self.height - 1 {
                let state = self.get_state(i, j);
                if state == 0 {
                    continue;
                };
                if current_state == state {
                    counter += 1;
                } else {
                    counter = 1;
                    current_state = state;
                }
                if counter == target_length {
                    return current_state;
                }
            }
            counter = 0;
        }
        return 0;
    }

    fn diagonal_check(&self) -> u8 {
        // x coord
        let target_len: u8 = 3;
        let mut counter: u8 = 0;
        let mut current_state: u8 = 0;
        for i in 0..=self.width {
            if !self.is_oob(i, i) {
                continue;
            }
            let state: u8 = self.get_state(i, i);
            if state == 0 {
                continue;
            }
            if state == current_state {
                counter += 1;
            } else {
                counter = 1;
                current_state = state;
            }
            if counter == target_len {
                return state;
            };
        }
        counter = 0;
        current_state = 0;
        // y coord
        for i in 1..=self.height {
            if !self.is_oob(i, i) {
                continue;
            }
            let state: u8 = self.get_state(i, i);
            if state == 0 {
                continue;
            }
            if state == current_state {
                counter += 1;
            } else {
                counter = 1;
                current_state = state;
            }
            if counter == target_len {
                return state;
            };
        }
        return 0;
    }

    fn is_oob(&self, width: u32, height: u32) -> bool {
        return width < self.width && height < self.height;
    }
    pub fn check_board(&self) -> u8 {
        //vertical check
        let vert = self.vertical_check();
        if vert != 0 {
            println!("Vertical Win Discovered: {}", vert);
            return vert;
        }
        println!("Vert Check Done");
        //horizontal check
        let hor = self.horizontal_check();
        if hor != 0 {
            println!("Horizontal Win Discovered: {}", hor);
            return hor;
        }
        println!("Hor Check Done");
        //diagonal check
        let res = self.diagonal_check();
        if res != 0 {
            println!("Diagonal Win Discovered: {}", res);
            return res;
        }
        println!("Dia Check Done");
        return 0;
    }
}

pub struct BiRange {
    start: usize,
    end: usize,
    step: isize,
}

impl BiRange {
    pub fn new(start: usize, end: usize) -> Self {
        if start < end {
            Self { start, end, step: 1 }
        } else {
            Self { start, end, step: -1 }
        }
    }
}

// This is a custom iterator that goes from start to end, both excluded, no matter if start is bigger than end
impl Iterator for BiRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.start = (self.start as isize + self.step) as usize;

            if self.start == self.end {
                None
            } else {
                Some(self.start)
            }
        }
    }
}

#[allow(dead_code)]
pub fn parse_move(input: &str) -> Result<((usize, usize), (usize, usize)), &'static str> {
    let input: Vec<&str> = input.trim().split(" ").collect();
    if input.len() != 2 {
        return Err("Invalid input");
    }

    let start = input[0];
    let end = input[1];

    if start.len() != 2 || end.len() != 2 {
        return Err("Invalid input");
    }

    let start: Vec<char> = start.chars().collect();
    let end: Vec<char> = end.chars().collect();

    let start = (
        start[1].to_digit(10).ok_or("Invalid input")? as usize - 1,
        start[0] as usize - 'a' as usize,
    );

    let end = (
        end[1].to_digit(10).ok_or("Invalid input")? as usize - 1,
        end[0] as usize - 'a' as usize,
    );

    Ok((start, end))
}
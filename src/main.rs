#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Colour {
    Green,
    Blue,
    Yellow,
    White,
    Red,
    Orange,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Face {
    ul: Colour,
    um: Colour,
    ur: Colour,
    ml: Colour,
    mm: Colour,
    mr: Colour,
    bl: Colour,
    bm: Colour,
    br: Colour,
}

/// Ordering of contents of lines are in clockwise order
#[derive(Clone, PartialEq, Eq, Debug)]
struct Line(Colour, Colour, Colour);

impl Face {
    pub fn fill(colour: Colour) -> Self {
        Face {
            ul: colour,
            um: colour,
            ur: colour,
            ml: colour,
            mm: colour,
            mr: colour,
            bl: colour,
            bm: colour,
            br: colour,
        }
    }
    pub fn rot_clockwise(&mut self) {
        let new_face = Face {
            ul: self.bl,
            um: self.ml,
            ur: self.ul,
            ml: self.bm,
            mm: self.mm,
            mr: self.um,
            bl: self.br,
            bm: self.mr,
            br: self.ur,
        };
        *self = new_face;
    }
    pub fn rot_counterclockwise(&mut self) {
        let new_face = Face {
            ul: self.ur,
            um: self.mr,
            ur: self.br,
            ml: self.um,
            mm: self.mm,
            mr: self.bm,
            bl: self.ul,
            bm: self.ml,
            br: self.bl,
        };
        *self = new_face;
    }
    // ordering of contents of lines are in clockwise order
    pub fn get_left_line(&self) -> Line {
        Line(self.bl, self.ml, self.ul)
    }
    pub fn get_upper_line(&self) -> Line {
        Line(self.ul, self.um, self.ur)
    }
    pub fn get_right_line(&self) -> Line {
        Line(self.ur, self.mr, self.br)
    }
    pub fn get_bottom_line(&self) -> Line {
        Line(self.br, self.bm, self.bl)
    }
    pub fn set_left_line(&mut self, line: Line) {
        self.bl = line.0;
        self.ml = line.1;
        self.ul = line.2;
    }
    pub fn set_upper_line(&mut self, line: Line) {
        self.ul = line.0;
        self.um = line.1;
        self.ur = line.2;
    }
    pub fn set_right_line(&mut self, line: Line) {
        self.ur = line.0;
        self.mr = line.1;
        self.br = line.2;
    }
    pub fn set_bottom_line(&mut self, line: Line) {
        self.br = line.0;
        self.bm = line.1;
        self.bl = line.2;
    }
}

struct Cube {
    up: Face,
    front: Face,
    left: Face,
    back: Face,
    right: Face,
    down: Face,
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            up: Face::fill(Colour::White),
            front: Face::fill(Colour::Green),
            left: Face::fill(Colour::Orange),
            back: Face::fill(Colour::Blue),
            right: Face::fill(Colour::Red),
            down: Face::fill(Colour::Yellow),
        }
    }
}

impl Cube {
    pub fn turn_right(&mut self) {
        self.right.rot_clockwise();
        let front_right_line: Line = self.front.get_right_line();
        let up_right_line: Line = self.up.get_right_line();
        let back_left_line: Line = self.back.get_left_line();
        let down_left_line: Line = self.down.get_left_line();
        self.up.set_right_line(front_right_line);
        self.back.set_left_line(up_right_line);
        self.down.set_left_line(back_left_line);
        self.front.set_right_line(down_left_line);
    }
    pub fn turn_right_prime(&mut self) {
        self.right.rot_clockwise();
        let front_right_line: Line = self.front.get_right_line();
        let up_right_line: Line = self.up.get_right_line();
        let back_left_line: Line = self.back.get_left_line();
        let down_left_line: Line = self.down.get_left_line();
        self.down.set_right_line(front_right_line);
        self.front.set_left_line(up_right_line);
        self.up.set_left_line(back_left_line);
        self.back.set_right_line(down_left_line);
    }
    pub fn turn_front(&mut self) {
        self.front.rot_clockwise();
        let left_right_line: Line = self.left.get_right_line();
        let up_bottom_line: Line = self.up.get_bottom_line();
        let right_left_line: Line = self.right.get_left_line();
        let down_bottom_line: Line = self.down.get_bottom_line();
        self.up.set_bottom_line(left_right_line);
        self.right.set_left_line(up_bottom_line);
        self.down.set_bottom_line(right_left_line);
        self.left.set_right_line(down_bottom_line);
    }
    pub fn turn_front_prime(&mut self) {
        self.front.rot_clockwise();
        let left_right_line: Line = self.left.get_right_line();
        let up_bottom_line: Line = self.up.get_bottom_line();
        let right_left_line: Line = self.right.get_left_line();
        let down_bottom_line: Line = self.down.get_bottom_line();
        self.down.set_bottom_line(left_right_line);
        self.left.set_left_line(up_bottom_line);
        self.up.set_bottom_line(right_left_line);
        self.right.set_right_line(down_bottom_line);
    }
    pub fn turn_left(&mut self) {
        self.left.rot_clockwise();
        let back_right_line: Line = self.back.get_right_line();
        let up_left_line: Line = self.up.get_left_line();
        let front_left_line: Line = self.front.get_left_line();
        let down_right_line: Line = self.down.get_right_line();
        self.up.set_left_line(back_right_line);
        self.front.set_left_line(up_left_line);
        self.down.set_right_line(front_left_line);
        self.back.set_right_line(down_right_line);
    }
    pub fn turn_left_prime(&mut self) {
        self.left.rot_clockwise();
        let back_right_line: Line = self.back.get_right_line();
        let up_left_line: Line = self.up.get_left_line();
        let front_left_line: Line = self.front.get_left_line();
        let down_right_line: Line = self.down.get_right_line();
        self.down.set_left_line(back_right_line);
        self.back.set_left_line(up_left_line);
        self.up.set_right_line(front_left_line);
        self.front.set_right_line(down_right_line);
    }
    pub fn turn_back(&mut self) {
        self.back.rot_clockwise();
        let up_upper_line: Line = self.up.get_upper_line();
        let left_left_line: Line = self.left.get_left_line();
        let down_upper_line: Line = self.down.get_upper_line();
        let right_right_line: Line = self.right.get_right_line();
        self.left.set_left_line(up_upper_line);
        self.down.set_upper_line(left_left_line);
        self.right.set_right_line(down_upper_line);
        self.up.set_upper_line(right_right_line);
    }
    pub fn turn_back_prime(&mut self) {
        self.back.rot_clockwise();
        let up_upper_line: Line = self.up.get_upper_line();
        let left_left_line: Line = self.left.get_left_line();
        let down_upper_line: Line = self.down.get_upper_line();
        let right_right_line: Line = self.right.get_right_line();
        self.right.set_left_line(up_upper_line);
        self.up.set_upper_line(left_left_line);
        self.left.set_right_line(down_upper_line);
        self.down.set_upper_line(right_right_line);
    }
    pub fn turn_down(&mut self) {
        self.down.rot_clockwise();
        let left_bottom_line: Line = self.left.get_bottom_line();
        let back_bottom_line: Line = self.back.get_bottom_line();
        let right_bottom_line: Line = self.right.get_bottom_line();
        let front_bottom_line: Line = self.front.get_bottom_line();
        self.back.set_bottom_line(right_bottom_line);
        self.right.set_bottom_line(front_bottom_line);
        self.front.set_bottom_line(left_bottom_line);
        self.left.set_bottom_line(back_bottom_line);
    }
    pub fn turn_down_prime(&mut self) {
        self.down.rot_clockwise();
        let left_bottom_line: Line = self.left.get_bottom_line();
        let back_bottom_line: Line = self.back.get_bottom_line();
        let right_bottom_line: Line = self.right.get_bottom_line();
        let front_bottom_line: Line = self.front.get_bottom_line();
        self.front.set_bottom_line(right_bottom_line);
        self.left.set_bottom_line(front_bottom_line);
        self.back.set_bottom_line(left_bottom_line);
        self.right.set_bottom_line(back_bottom_line);
    }
    pub fn turn_up(&mut self) {
        self.up.rot_clockwise();
        let left_upper_line: Line = self.left.get_upper_line();
        let back_upper_line: Line = self.back.get_upper_line();
        let right_upper_line: Line = self.right.get_upper_line();
        let front_upper_line: Line = self.front.get_upper_line();
        self.back.set_upper_line(left_upper_line);
        self.right.set_upper_line(back_upper_line);
        self.front.set_upper_line(right_upper_line);
        self.left.set_upper_line(front_upper_line);
    }
    pub fn turn_up_prime(&mut self) {
        self.up.rot_clockwise();
        let left_upper_line: Line = self.left.get_upper_line();
        let back_upper_line: Line = self.back.get_upper_line();
        let right_upper_line: Line = self.right.get_upper_line();
        let front_upper_line: Line = self.front.get_upper_line();
        self.front.set_upper_line(left_upper_line);
        self.left.set_upper_line(back_upper_line);
        self.back.set_upper_line(right_upper_line);
        self.right.set_upper_line(front_upper_line);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotation() {
        let initial_face = Face {
            ul: Colour::Blue,
            um: Colour::Red,
            ur: Colour::Yellow,
            ml: Colour::Orange,
            mm: Colour::White,
            mr: Colour::Blue,
            bl: Colour::Yellow,
            bm: Colour::White,
            br: Colour::Red,
        };
        let mut face1 = initial_face.clone();
        let mut face2 = initial_face.clone();
        let clock_turned_face = Face {
            ul: Colour::Yellow,
            um: Colour::Orange,
            ur: Colour::Blue,
            ml: Colour::White,
            mm: Colour::White,
            mr: Colour::Red,
            bl: Colour::Red,
            bm: Colour::Blue,
            br: Colour::Yellow,
        };
        let cc_turned_face = Face {
            ul: Colour::Yellow,
            um: Colour::Blue,
            ur: Colour::Red,
            ml: Colour::Red,
            mm: Colour::White,
            mr: Colour::White,
            bl: Colour::Blue,
            bm: Colour::Orange,
            br: Colour::Yellow,
        };
        face1.rot_clockwise();
        face2.rot_counterclockwise();
        assert_eq!(face1, clock_turned_face);
        assert_eq!(face2, cc_turned_face);
    }
}

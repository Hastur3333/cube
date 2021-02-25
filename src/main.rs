pub mod algoritm;

use algoritm::*;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Colour {
    Green,
    Blue,
    Yellow,
    White,
    Red,
    Orange,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Green => write!(f, "G")?,
            Self::Blue => write!(f, "B")?,
            Self::Yellow => write!(f, "Y")?,
            Self::White => write!(f, "W")?,
            Self::Red => write!(f, "R")?,
            Self::Orange => write!(f, "O")?,
        }

        Ok(())
    }
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

impl Display for Face {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "{}|{}|{}", self.ul, self.um, self.ur)?;
        writeln!(f, "-+-+-")?;
        writeln!(f, "{}|{}|{}", self.ml, self.mm, self.mr)?;
        writeln!(f, "-+-+-");
        writeln!(f, "{}|{}|{}", self.bl, self.bm, self.br)?;

        Ok(())
    }
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

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "      {}|{}|{}", self.up.ul, self.up.um, self.up.ur);
        writeln!(f, "      -+-+-");
        writeln!(f, "      {}|{}|{}", self.up.ml, self.up.mm, self.up.mr);
        writeln!(f, "      -+-+-");
        writeln!(f, "      {}|{}|{}", self.up.bl, self.up.bm, self.up.br);
        writeln!(f, "");
        writeln!(
            f,
            "{}|{}|{} {}|{}|{} {}|{}|{} {}|{}|{}",
            self.left.ul,
            self.left.um,
            self.left.ur,
            self.front.ul,
            self.front.um,
            self.front.ur,
            self.right.ul,
            self.right.um,
            self.right.ur,
            self.back.ul,
            self.back.um,
            self.back.ur
        );
        writeln!(f, "-+-+- -+-+- -+-+- -+-+-");
        writeln!(
            f,
            "{}|{}|{} {}|{}|{} {}|{}|{} {}|{}|{}",
            self.left.ml,
            self.left.mm,
            self.left.mr,
            self.front.ml,
            self.front.mm,
            self.front.mr,
            self.right.ml,
            self.right.mm,
            self.right.mr,
            self.back.ml,
            self.back.mm,
            self.back.mr
        );
        writeln!(f, "-+-+- -+-+- -+-+- -+-+-");
        writeln!(
            f,
            "{}|{}|{} {}|{}|{} {}|{}|{} {}|{}|{}",
            self.left.bl,
            self.left.bm,
            self.left.br,
            self.front.bl,
            self.front.bm,
            self.front.br,
            self.right.bl,
            self.right.bm,
            self.right.br,
            self.back.bl,
            self.back.bm,
            self.back.br
        );
        writeln!(f, "");
        writeln!(
            f,
            "      {}|{}|{}",
            self.down.ul, self.down.um, self.down.ur
        );
        writeln!(f, "      -+-+-");
        writeln!(
            f,
            "      {}|{}|{}",
            self.down.ml, self.down.mm, self.down.mr
        );
        writeln!(f, "      -+-+-");
        writeln!(
            f,
            "      {}|{}|{}",
            self.down.bl, self.down.bm, self.down.br
        );

        Ok(())
    }
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
    pub fn get_face(&self, side: Side) -> &Face {
        match side {
            Side::Up => &self.up,
            Side::Down => &self.down,
            Side::Front => &self.front,
            Side::Back => &self.back,
            Side::Right => &self.right,
            Side::Left => &self.left,
        }
    }

    pub fn get_face_mut(&mut self, side: Side) -> &mut Face {
        match side {
            Side::Up => &mut self.up,
            Side::Down => &mut self.down,
            Side::Front => &mut self.front,
            Side::Back => &mut self.back,
            Side::Right => &mut self.right,
            Side::Left => &mut self.left,
        }
    }

    pub fn get_up_lines(&self) -> [Line; 4] {
        [
            self.front.get_upper_line(),
            self.left.get_upper_line(),
            self.back.get_upper_line(),
            self.right.get_upper_line(),
        ]
    }

    pub fn get_down_lines(&self) -> [Line; 4] {
        [
            self.front.get_bottom_line(),
            self.left.get_bottom_line(),
            self.back.get_bottom_line(),
            self.right.get_bottom_line(),
        ]
    }

    pub fn get_front_lines(&self) -> [Line; 4] {
        [
            self.up.get_bottom_line(),
            self.right.get_left_line(),
            self.down.get_upper_line(),
            self.left.get_right_line(),
        ]
    }

    pub fn get_back_lines(&self) -> [Line; 4] {
        [
            self.up.get_upper_line(),
            self.right.get_right_line(),
            self.down.get_bottom_line(),
            self.left.get_left_line(),
        ]
    }

    pub fn get_right_lines(&self) -> [Line; 4] {
        [
            self.front.get_right_line(),
            self.up.get_right_line(),
            self.back.get_left_line(),
            self.down.get_right_line(),
        ]
    }

    pub fn get_left_lines(&self) -> [Line; 4] {
        [
            self.front.get_left_line(),
            self.up.get_left_line(),
            self.back.get_right_line(),
            self.down.get_left_line(),
        ]
    }

    pub fn set_up_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_upper_line(a);
        self.left.set_upper_line(b);
        self.back.set_upper_line(c);
        self.right.set_upper_line(d);
    }

    pub fn set_down_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_bottom_line(a);
        self.left.set_bottom_line(b);
        self.back.set_bottom_line(c);
        self.right.set_bottom_line(d);
    }

    pub fn set_front_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_bottom_line(a);
        self.right.set_left_line(b);
        self.down.set_upper_line(c);
        self.left.set_right_line(d);
    }

    pub fn set_back_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_upper_line(a);
        self.right.set_right_line(b);
        self.down.set_bottom_line(c);
        self.left.set_left_line(d);
    }

    pub fn set_right_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_right_line(a);
        self.up.set_right_line(b);
        self.back.set_left_line(c);
        self.down.set_right_line(d);
    }

    pub fn set_left_lines(&mut self, [a, b, c, d]: [Line; 4]) {
        self.front.set_left_line(a);
        self.up.set_left_line(b);
        self.back.set_right_line(c);
        self.down.set_left_line(d);
    }

    pub fn get_lines(&self, side: Side) -> [Line; 4] {
        match side {
            Side::Up => self.get_up_lines(),
            Side::Down => self.get_down_lines(),
            Side::Front => self.get_front_lines(),
            Side::Back => self.get_back_lines(),
            Side::Right => self.get_right_lines(),
            Side::Left => self.get_left_lines(),
        }
    }

    pub fn set_lines(&mut self, side: Side, lines: [Line; 4]) {
        match side {
            Side::Up => self.set_up_lines(lines),
            Side::Down => self.set_down_lines(lines),
            Side::Front => self.set_front_lines(lines),
            Side::Back => self.set_back_lines(lines),
            Side::Right => self.set_right_lines(lines),
            Side::Left => self.set_left_lines(lines),
        }
    }

    pub fn turn_side(&mut self, side: Side) {
        self.get_face_mut(side.clone()).rot_clockwise();

        let mut lines = self.get_lines(side.clone());
        lines.rotate_right(1);
        self.set_lines(side, lines);
    }

    pub fn turn_side_prime(&mut self, side: Side) {
        self.get_face_mut(side.clone()).rot_counterclockwise();

        let mut lines = self.get_lines(side.clone());
        lines.rotate_left(1);
        self.set_lines(side, lines);
    }

    pub fn turn_side_twice(&mut self, side: Side) {
        let face = self.get_face_mut(side.clone());
        face.rot_clockwise();
        face.rot_clockwise();

        let mut lines = self.get_lines(side.clone());
        lines.rotate_right(2);
        self.set_lines(side, lines);
    }

    // these functions shouldn't be needed, review before removing
    //      Hjalte 25-02-21
    /*
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
        self.down.set_left_line(front_right_line);
        self.front.set_right_line(up_right_line);
        self.up.set_right_line(back_left_line);
        self.back.set_right_line(down_left_line);
    }
    pub fn turn_right_twice(&mut self) {
        self.right.rot_clockwise();
        let front_right_line: Line = self.front.get_right_line();
        let up_right_line: Line = self.up.get_right_line();
        let back_left_line: Line = self.back.get_left_line();
        let down_left_line: Line = self.down.get_left_line();
        self.back.set_left_line(front_right_line);
        self.down.set_left_line(up_right_line);
        self.front.set_right_line(back_left_line);
        self.up.set_right_line(down_left_line);
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
    */

    pub fn execute_move(&mut self, mv: Move) {
        match &mv.ty {
            MoveType::Normal => self.turn_side(mv.side.clone()),
            MoveType::Prime => self.turn_side_prime(mv.side.clone()),
            MoveType::Double => self.turn_side_twice(mv.side.clone()),
        }
    }

    pub fn execute_algorithm(&mut self, alg: Algorithm) {
        for mv in alg.moves {
            self.execute_move(mv);
        }
    }
}

fn main() {
    let mut cube = Cube::default();

    println!("{}", cube);

    let alg = Algorithm::parse("R U R' U'").unwrap();

    println!("{:?}", alg);

    cube.execute_algorithm(alg);

    println!("{}", cube);
}

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

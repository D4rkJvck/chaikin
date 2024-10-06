const RADIUS: i32 = 10;

pub struct Point {
    x: i32,
    y: i32,
    ctrl: bool,
}

impl Point {
    pub fn new(x: i32, y: i32, ctrl: bool) -> Self {
        Point { x, y, ctrl }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn is_ctrl_point(&self) -> bool {
        self.ctrl
    }
}

//________________________________________________________________
//

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    pub fn get_a(&self) -> &Point {
        &self.a
    }

    pub fn get_b(&self) -> &Point {
        &self.b
    }
}

//________________________________________________________________
//

pub struct Circle {
    center: Point,
}

impl Circle {
    pub fn new(center: Point) -> Self {
        Circle { center }
    }

    pub fn get_center(&self) -> &Point {
        &self.center
    }
}

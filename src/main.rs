fn main() {
    println!("Hello, world!");
	let p1 = Position::new(0f64,0f64);
	let p2 = Position::new(1f64, 1f64);

	print!("{}", distance(p1, p2));
}

fn distance(p1: Position, p2: Position) -> f64 {
	let d = ((p2.x-p1.x).powf(2f64)+(p2.y-p1.y).powf(2f64)).sqrt();
	d
}

struct Circle {
	pub pos: Position,
	pub radius: f64,
}


struct Position {
	pub x: f64,
	pub y: f64,
}

impl Position {
	pub fn new(x: f64, y: f64) -> Position {
		Position {
			x,
			y,
		}
	}
}
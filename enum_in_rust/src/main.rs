enum Direction {
    Up,
    Down,
    Left,
    Right
}


fn main() {
let plarer_direction:Direction=Direction::Up;
match plarer_direction{
    Direction::Up=>println!("Player direction is Up"),
    Direction::Down=>println!("Player direction is Down"),
    Direction::Left=>println!("Player direction is Left"),
    Direction::Right=>println!("Player direction is Right")
}

}

#[derive(PartialEq)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

// Vec3 normalization not required
pub fn vec_to_dir(vec:Vec3) -> Option<Direction> {
    
    // Leftwards
    if vec.x < 0 {
        // Downwards
        if vec.y < 0 {
            return Some(Direction::DownLeft);
        }
        // Upwards
        else if vec.y > 0 {
            return Some(Direction::UpLeft);
        }
        // No vertical component
        else {
            return Some(Direction::Left);
        }
    } 

    // Rightwards
    else if x > 0 {
        // Downwards
        if vec.y < 0 {
            return Some(Direction::DownRight);
        }
        // Upwards
        else if vec.y > 0 {
            return Some(Direction::UpRight);
        }
        // No vertical component
        else {
            return Some(Direction::Right);
        }

    } 
    
    // No horizontal component
    else {
        // Downwards
        if vec.y < 0 {
            return Some(Direction::Down);
        }
        // Upwards
        else if vec.y > 0 {
            return Some(Direction::Up);
        }
        // No vertical component
        else {
            return None;
        }

    }

}
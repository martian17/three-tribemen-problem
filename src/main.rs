#[derive(PartialEq, Copy, Clone)]
enum Tribesman {
    Random,
    Truth,
    False,
}

impl std::fmt::Display for Tribesman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tribesman::Random => write!(f, "Random"),
            Tribesman::Truth => write!(f, "Truth"),
            Tribesman::False => write!(f, "False"),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl Tribesman {
    fn eval(&self, val: bool) -> bool {
        match self {
            Tribesman::Random => rand::random_bool(0.5),
            Tribesman::Truth => val,
            Tribesman::False => !val,
        }
    }
    fn ask_equal<T: PartialEq>(&self, v1: T, v2: T) -> bool {
        match self {
            Tribesman::Random => rand::random_bool(0.5),
            Tribesman::Truth => v1 == v2,
            Tribesman::False => v1 != v2,
        }
    }
}



fn classic_problem() {
    // initialization
    let (tribesman_1, tribesman_2) = if rand::random_bool(0.5) {
        (Tribesman::Truth, Tribesman::False)
    } else {
        (Tribesman::False, Tribesman::Truth)
    };

    let direction_heaven = if rand::random_bool(0.5) {
        Direction::Left
    } else {
        Direction::Right
    };



    // the solution
    let success = if tribesman_1.eval(tribesman_1.ask_equal(Direction::Left, direction_heaven)) {
        println!("Left leads to heaven");
        direction_heaven == Direction::Left
    } else {
        println!("Right leads to heaven");
        direction_heaven == Direction::Right
    };

    if success {
        print!("Success! ");
    } else {
        print!("Failure! ");
    }
    println!("tribesman_1: {}, direction_heaven: {}", tribesman_1, direction_heaven);
}

fn three_tribesmen_problem() {
    // initialization
    let (tribesman_1, tribesman_2, tribesman_3) = if rand::random_bool(1.0/6.0) {
        (Tribesman::Truth, Tribesman::False, Tribesman::Random)
    } else if rand::random_bool(1.0/5.0) {
        (Tribesman::Truth, Tribesman::Random, Tribesman::False)
    } else if rand::random_bool(1.0/4.0) {
        (Tribesman::False, Tribesman::Truth, Tribesman::Random)
    } else if rand::random_bool(1.0/3.0) {
        (Tribesman::False, Tribesman::Random, Tribesman::Truth)
    } else if rand::random_bool(1.0/2.0) {
        (Tribesman::Random, Tribesman::Truth, Tribesman::False)
    } else {
        (Tribesman::Random, Tribesman::False, Tribesman::Truth)
    };

    let direction_heaven = if rand::random_bool(0.5) {
        Direction::Left
    } else {
        Direction::Right
    };

    // idea: iff there is a random should the answer be different
    // in that case the 
    //
    //
    // no, thought for about 10 minutes, and the first idea doesn't work
    //
    // idea 2: ask the first one to choose the second one, then ask the second one the real question

    let first_response = tribesman_1.eval(tribesman_1.ask_equal(tribesman_2, Tribesman::Random));

    let q2_subject = if first_response {
        // tribesman_2 is random.
        tribesman_3
    } else {
        // tribesman_3 is random.
        tribesman_2
    };

    let second_response = q2_subject.eval(q2_subject.ask_equal(Direction::Left, direction_heaven));

    let success = if second_response {
        println!("Left leads to heaven");
        direction_heaven == Direction::Left
    } else {
        println!("Right leads to heaven");
        direction_heaven == Direction::Right
    };

    if success {
        print!("Success! ");
    } else {
        print!("Failure! ");
    }
    println!("tribesman_1: {}, tribesman_2: {}, tribesman_3: {}, q2_subject: {}, direction_heaven: {}", tribesman_1, tribesman_2, tribesman_3, q2_subject, direction_heaven);
}


fn main() {
    // for i in 0..10 {
    //     println!("\nRound {}", i);
    //     classic_problem();
    // }

    for i in 0..10 {
        println!("\nRound {}", i);
        three_tribesmen_problem();
    }
}

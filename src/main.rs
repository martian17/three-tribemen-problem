#[derive(PartialEq, Copy, Clone)]
enum Tribeman {
    Random,
    Truth,
    False,
}

impl std::fmt::Display for Tribeman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tribeman::Random => write!(f, "Random"),
            Tribeman::Truth => write!(f, "Truth"),
            Tribeman::False => write!(f, "False"),
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

impl Tribeman {
    fn eval(&self, val: bool) -> bool {
        match self {
            Tribeman::Random => rand::random_bool(0.5),
            Tribeman::Truth => val,
            Tribeman::False => !val,
        }
    }
    fn ask_equal<T: PartialEq>(&self, v1: T, v2: T) -> bool {
        match self {
            Tribeman::Random => rand::random_bool(0.5),
            Tribeman::Truth => v1 == v2,
            Tribeman::False => v1 != v2,
        }
    }
}



fn classic_problem() {
    // initialization
    let (tribeman_1, tribeman_2) = if rand::random_bool(0.5) {
        (Tribeman::Truth, Tribeman::False)
    } else {
        (Tribeman::False, Tribeman::Truth)
    };

    let direction_heaven = if rand::random_bool(0.5) {
        Direction::Left
    } else {
        Direction::Right
    };



    // the solution
    let success = if tribeman_1.eval(tribeman_1.ask_equal(Direction::Left, direction_heaven)) {
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
    println!("tribeman_1: {}, direction_heaven: {}", tribeman_1, direction_heaven);
}

fn three_tribemen_problem() {
    // initialization
    let (tribeman_1, tribeman_2, tribeman_3) = if rand::random_bool(1.0/6.0) {
        (Tribeman::Truth, Tribeman::False, Tribeman::Random)
    } else if rand::random_bool(1.0/5.0) {
        (Tribeman::Truth, Tribeman::Random, Tribeman::False)
    } else if rand::random_bool(1.0/4.0) {
        (Tribeman::False, Tribeman::Truth, Tribeman::Random)
    } else if rand::random_bool(1.0/3.0) {
        (Tribeman::False, Tribeman::Random, Tribeman::Truth)
    } else if rand::random_bool(1.0/2.0) {
        (Tribeman::Random, Tribeman::Truth, Tribeman::False)
    } else {
        (Tribeman::Random, Tribeman::False, Tribeman::Truth)
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

    let first_response = tribeman_1.eval(tribeman_1.ask_equal(tribeman_2, Tribeman::Random));

    let q2_subject = if first_response {
        // tribeman_2 is random.
        tribeman_3
    } else {
        // tribeman_3 is random.
        tribeman_2
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
    println!("tribeman_1: {}, tribeman_2: {}, tribeman_3: {}, q2_subject: {}, direction_heaven: {}", tribeman_1, tribeman_2, tribeman_3, q2_subject, direction_heaven);
}


fn main() {
    // for i in 0..10 {
    //     println!("\nRound {}", i);
    //     classic_problem();
    // }

    for i in 0..10 {
        println!("\nRound {}", i);
        three_tribemen_problem();
    }
}

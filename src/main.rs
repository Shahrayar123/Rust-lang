// mod print_formatting;
// mod variables;
// mod data_types;
// mod string;
// mod tuples;
// mod array;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod structure;


enum Direction{
    UP,
    DOWN,
    RIGHT,
    LEFT
}


fn main() {
    // print_formatting::message();
    // variables::understand_variables();
    // data_types::types();
    // string::string();
    // tuples::tuple();
    // array::array();
    // vectors::vector();
    // conditionals::condition();
    // loops::run();

    // functions::run();
    // structure::run();

    let player_direction : Direction = Direction::LEFT;

    match player_direction{
        Direction::UP => println!("Moving up"),
        Direction::DOWN => println!("Moving down"),
        Direction::RIGHT => println!("Moving right"),
        Direction::LEFT => println!("Moving left")
    }

}


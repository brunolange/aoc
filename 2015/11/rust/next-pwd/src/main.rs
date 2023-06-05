use next_pwd::{next_password, next_password_fast};

mod io;

fn main() {
    env_logger::init();

    let next = if io::fast() {
        next_password_fast
    } else {
        next_password
    };

    match next(&io::read_password()) {
        None => panic!("There is no viable next password"),
        Some(pwd) => println!("{}", pwd),
    };
}

use next_pwd::next_password;

mod io;

fn main() {
    env_logger::init();

    match next_password(&io::read_password()) {
        None => panic!("There is no viable next password"),
        Some(pwd) => println!("{}", pwd),
    };
}

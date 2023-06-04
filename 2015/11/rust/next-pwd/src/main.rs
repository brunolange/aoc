use next_pwd::next_password;

mod io;

fn main() {
    env_logger::init();
    let password = io::read_password();
    match next_password(&password) {
        None => panic!("Could not find a viable next password"),
        Some(pwd) => println!("{}", pwd),
    };
}

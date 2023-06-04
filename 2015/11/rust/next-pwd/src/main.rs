use next_pwd::{is_valid_password, Password, PasswordIterator};

mod io;

fn main() {
    env_logger::init();
    let pwd: Password<8> =
        Password::from_str(io::read_password().as_str()).expect("invalid password");

    let mut pi = PasswordIterator { pwd };

    let next = pi.find(is_valid_password);

    let next_password = next.expect("could not find next password");
    println!("{}", next_password.value.iter().collect::<String>());
}

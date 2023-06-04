use next_pwd::{Password, PasswordIterator};

mod io;

fn main() {
    let pwd: Password<8> =
        Password::from_str(io::read_password().as_str()).expect("invalid password");

    let mut pi = PasswordIterator { pwd };

    let next = pi.find(|p| p.value[7] == 'z');
    println!("next = {:?}", next)
}

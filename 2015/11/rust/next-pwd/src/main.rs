use next_pwd::{Password, PasswordIterator};

mod io;

fn main() {
    let pwd: Password<8> =
        Password::from_str(io::read_password().as_str()).expect("invalid password");

    let pi = PasswordIterator { pwd };
    let mut counter = 0;
    for n in pi {
        println!("{:?}", n);
        counter += 1;
        if counter == 100 {
            break;
        }
    }
}

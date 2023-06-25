use next_pwd::next_passwords;

mod io;

fn main() {
    env_logger::init();

    for next_pwd in next_passwords(&io::read_password(), io::number_of_passwords()) {
        println!("{}", next_pwd);
    }
}

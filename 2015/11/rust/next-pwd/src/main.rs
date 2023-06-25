use next_pwd::next_passwords;

mod io;

fn main() {
    env_logger::init();

    let mut found = false;
    for next_pwd in next_passwords(&io::read_password(), io::number_of_passwords()) {
        found = true;
        println!("{}", next_pwd);
    }

    if !found {
        eprintln!("Error: There is no next viable password");
        std::process::exit(1);
    }
}

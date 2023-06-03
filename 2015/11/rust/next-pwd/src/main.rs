#[derive(Debug)]
struct PasswordIterator {
    pwd: [char; 8],
}

impl Iterator for PasswordIterator {
    type Item = [char; 8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut nxt = self.pwd.clone();

        let mut carry = true;
        let mut i = 7;
        while carry {
            (nxt[i], carry) = inc(nxt[i]);
            i -= 1;
        }

        let n = if carry && i == 0 { None } else { Some(nxt) };

        self.pwd = nxt;

        n
    }
}

fn inc(c: char) -> (char, bool) {
    let carry = c == 'z';
    let nxt = if carry {
        'a'
    } else {
        ((c as u32) + 1).try_into().unwrap()
    };
    (nxt, carry)
}

fn main() {
    let pwd = ['h', 'x', 'b', 'x', 'w', 'x', 'b', 'a'];
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

use nom::{bytes::complete::take_while_m_n, combinator::all_consuming, IResult};

pub fn parse_pwd<const N: usize>(input: &str) -> IResult<&str, &str> {
    let (remaining, pwd) = all_consuming(take_while_m_n(N, N, |c: char| c.is_lowercase()))(input)?;
    Ok((remaining, pwd))
}

#[derive(Clone, Debug)]
pub struct Password<const N: usize> {
    value: [char; N],
}

impl<const N: usize> Password<N> {
    pub fn from_str(s: &str) -> Option<Self> {
        let (_, pwd) = parse_pwd::<N>(s).ok()?;
        let chars: Vec<_> = pwd.chars().collect();
        let pwd = chars.try_into().ok()?;
        Some(Password { value: pwd })
    }
}

#[derive(Debug)]
pub struct PasswordIterator<const N: usize> {
    pub pwd: Password<N>,
}

impl<const N: usize> Iterator for PasswordIterator<N> {
    type Item = Password<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut nxt = self.pwd.clone();

        let mut carry = true;
        let mut i: i32 = 7;
        while carry && i >= 0 {
            let idx = i as usize;
            (nxt.value[idx], carry) = inc(nxt.value[idx]);
            i -= 1;
        }

        self.pwd = nxt.clone();

        if carry && i == -1 {
            None
        } else {
            Some(nxt)
        }
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

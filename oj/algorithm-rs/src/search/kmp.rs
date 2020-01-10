fn generate_partial_match_table(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();

    let mut f = vec![0; s.len()];

    for i in 1..s.len() {
        let mut t = f[i - 1];
        while t > 0 && bytes[i] != bytes[t] {
            t = f[t - 1]
        }
        if bytes[i] == bytes[t] {
            t += 1;
        }
        f[i] = t;
    }

    f
}

#[allow(dead_code)]
pub fn search(s: &str, pat: &str) -> Vec<usize> {
    let f = generate_partial_match_table(pat);

    let s = s.as_bytes();
    let pat = pat.as_bytes();

    let mut i = 0;
    let mut j = 0;
    let mut res = vec![];

    while i < s.len() {
        if s[i] == pat[j] {
            i += 1;
            j += 1;
        }
        if j == pat.len() {
            res.push(i - j);

            j = f[j - 1];
        } else if i < s.len() && s[i] != pat[j] {
            if j != 0 {
                j = f[j - 1];
            } else {
                i += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn it_works() {
        assert_eq!(search("abcdefg", "acde"), vec![]);
        assert_eq!(search("aaaaaaa", "a"), vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(search("abcdefg", "cde"), vec![2]);
        assert_eq!(search("THIS IS A TEST TEXT", "TEST"), vec![10]);
        assert_eq!(search("AABAACAADAABAABA", "AABA"), vec![0, 9, 12]);
        assert_eq!(search("abbaabbaaba", "abbaaba"), vec![4]);
        assert_eq!(search("ababababca", "abababca"), vec![2]);
    }
}

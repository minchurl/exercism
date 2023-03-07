pub fn check(candidate: &str) -> bool {
    let mut cnt = [0; 26];
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| -> () {
            let i: usize = (c as usize) - 97;
            cnt[i] = cnt[i] + 1;
        })
    ;
    cnt.into_iter().all(|x| x < 2)
}

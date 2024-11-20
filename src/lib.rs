#![deny(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
// ---
#![cfg_attr(docsrs, feature(doc_cfg))]

#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

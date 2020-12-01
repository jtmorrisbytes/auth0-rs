// this attempts to be a faithful port of auth0js from
// https://github.com/auth0/auth0.js/

pub mod webauth;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

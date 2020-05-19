#[macro_export]
macro_rules! string_concat {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_string = String::new();
            $(
                temp_string.push_str($x);
            )*
            temp_string
        }
    };
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    #[test]
    fn it_works() {
        let s: String = string_concat!("hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn it_works_more_complicated() {
        let s: String = string_concat!("hello", "world");
        assert_eq!(s, "helloworld");
    }

    #[test]
    fn it_works_with_vars() {
        let name = "richard";
        let s: String = string_concat!("hello ", name, "!");
        assert_eq!(s, "hello richard!");
    }
}

#![no_std]

extern crate alloc;

#[doc(hidden)]
pub use alloc::string::String;

#[macro_export]
macro_rules! string_concat {
    ( $( $x:expr ),* ) => {{
        $crate::string_concat_impl!{[] $($x),*}
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! string_concat_impl {
    ([$($x:ident)*] ) => {{
        let mut temp_string = $crate::String::with_capacity(0$(+$x.len())*);
        $(
            temp_string.push_str($x);
        )*
        temp_string
    }};
    ([$($ident:ident)*] $x:expr $(, $rest:expr)*) => {
        let ref x = $x;
        string_concat_impl!{[$($ident)* x] $($rest),*}
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use crate::String;
    
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
    fn it_works_with_stringss() {
        let name = "richard".to_string();
        let s: String = string_concat!("hello ", name, "!");
        assert_eq!(s, "hello richard!");
    }

    #[test]
    fn it_works_with_str() {
        let name = "richard";
        let s: String = string_concat!("hello ", name, "!");
        assert_eq!(s, "hello richard!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(replace!("123", {
            "1" => "4",
            "2" => "5",
            "3" => "6"
        }), "456");
    }

   
}



#[macro_export]
macro_rules! replace {
    ($result:expr, { $( $key:expr => $value:expr ),* } ) => {
        {
            let mut a:String = $result.to_string();
            $(
                a = str::replace(&a, $key, $value);
            )*
            a
        }
    };
}
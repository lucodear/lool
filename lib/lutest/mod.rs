#[macro_export]
macro_rules! it {
    // Entry point for the macro, takes the struct definition
    (
        $fn_name:tt, $body:expr
        // $fn_name:ident $($rest:ident)+, $body:expr
    ) => {
        #[test]
        fn $fn_name () -> Result<(), Box<dyn std::error::Error>> {
            $body
        }
    };
}

// it!(should_do_the_trick, {
//     assert_eq!(2, 1);

//     Ok(())
// });

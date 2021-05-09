// macro_rules! eval {
//     ($($tt:tt)*) => {{
//         let js = stringify!($($tt)*);
//         todo!("implement compile time eval")
//     }};
// }

#[cfg(test)]
mod tests {
    use ctjs_macros::eval;

    #[test]
    fn it_works() {
        let y = eval! {
            let x = 3;
            x * 3.5
        };
        assert_eq!(y, 10.5);
    }
}

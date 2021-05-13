pub use ctjs_macros::*;

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

    #[test]
    fn it_can_generate_sin_table() {
        let nums: Vec<f64> = eval! {
            const values = Array.from({ length: 30 }, (x, i) => Math.sin(i / (Math.PI * 2)));
            "vec![" + values.map(value => value % 1 === 0 ? value + ".0" : value) + "]"
        };
        println!("Nums: {:#?}", nums);

        assert_eq!("making test fail to see stdout", "");
    }

    #[test]
    fn it_can_generate_sin_table_helpers() {
        let nums: Vec<f64> = eval! {
            const values = ctjs.range(0, 30).map(x => Math.sin(x / (Math.PI * 2)));
            ctjs.vec(values.map(ctjs.float))
        };
        println!("Nums: {:#?}", nums);

        assert_eq!("making test fail to see stdout", "");
    }

    #[test]
    fn it_can_generate_sin_table_raw_string() {
        let nums: Vec<f64> = eval! {r#"
            const values = Array.from({ length: 30 }, (x, i) => Math.sin(i / (Math.PI * 2)));
            `vec![${values.map(value => value % 1 === 0 ? value + ".0" : value)}]`
        "#};
        println!("Nums: {:#?}", nums);

        assert_eq!("making test fail to see stdout", "");
    }
}

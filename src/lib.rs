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
    fn it_handles_more_complexity() {
        let nums: Vec<f64> = eval! {
            const values = Array.from({ length: 30 }, (x, i) => Math.sin(i / (Math.PI * 2)));
            "vec![" + values.map(value => value % 1 ? value : value + ".0") + "]"
        };
        println!("Nums: {:#?}", nums);

        assert_eq!("making test fail to see stdout", "");
    }
}

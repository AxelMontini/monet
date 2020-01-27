#[cfg(feature = "csv")]
mod csv {
    use monet::Money;

    #[test]
    fn good() {
        mod currency {
            use monet::define_currency_csv;

            define_currency_csv!("monet/tests/good.csv");
        }

        let money = Money::<currency::USD>::with_amount(100);
    }
}


mod array {
    use monet::Money;
    
    #[test]
    fn array() {
        mod currency {
            use monet::define_currency_array;

            define_currency_array!([
                ("US Dollar", "USD", 2),
                ("Swiss Franc", "CHF", 2),
            ]);
        }
    }
}
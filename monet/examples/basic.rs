use monet::{Currency, Money};

mod currency {
    use monet::define_currency_csv;

    define_currency_csv!("monet/examples/currencies.csv");
}

fn main() {
    let money_1 = Money::<currency::IMC>::with_amount(12345);
    println!("Money 1: {}", money_1);

    let money_2 = Money::<currency::USD>::with_amount(54321);
    println!("Money 2: {}", money_2);
}

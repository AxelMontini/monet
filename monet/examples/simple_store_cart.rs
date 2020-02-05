use monet::{Money};

mod currency {
    monet::define_currency_csv!("monet/examples/currencies.csv");
}

struct Item {
    pub name: String,
    pub price: Money<currency::CHF>,
}

fn main() {
    let cart = cart();

    let total: Money<_> = cart.iter().map(|Item {price, ..}| price).sum();

    println!("Your cart");
    cart.iter().for_each(|item| println!("{:20} | {}", item.price, item.name));
    println!("{:-^30}", "TOTAL");
    println!("{}", total);
}

// Load items from a database or something
fn cart() -> Vec<Item> {
    vec![
        Item {name: "Soap".into(), price: Money::with_amount(500)},
        Item {name: "AMD Ryzen R9 3900x".into(), price: Money::with_amount(51500)},
        Item {name: "Some Item".into(), price: Money::with_amount(1850)},
        Item {name: "Bag".into(), price: Money::with_amount(50)},
        Item {name: "Discount".into(), price: Money::with_amount(-1500)},
    ]
}
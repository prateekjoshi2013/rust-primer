// Option implements Iterator trait

pub fn main() {
    let some_product = Some("laptop");
    let mut product_vec = vec!["cellphone", "battery", "charger"];

    match some_product {
        Some(product) => product_vec.push(product),
        _ => (),
    }

    if let Some(product) = some_product {
        product_vec.push(product)
    }

    // this code is equivalent to above code

    product_vec.extend(some_product);
    println!("{:?}", product_vec);

    // we can chain iterators and extend an
    // existing iterator with an option

    for product in product_vec.iter().chain(some_product.iter()) {
        println!("{:?}", product);
    }

    // when we want to Vec<Option<T>> -> to Vec<T> we can
    // use flatten method

    let products = vec![Some("Charger"), Some("Battery"), None, Some("Cellphone")];
    let filtered_products: Vec<&str> = products.clone().into_iter().flatten().collect();
    println!(
        "original :{:?} => flattened : {:?}",
        products, filtered_products
    );
}

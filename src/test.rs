use crate::Order;
use crate::Price;
use crate::*;
use std::collections::BTreeMap;

fn order_helper(id: OrderID, price: Price, side: Side) -> LimitOrder {
    LimitOrder {
        id,
        quantity: Quantity::new(1).unwrap(),
        price,
        side,
    }
}

#[test]
fn construct_bid_datastruct() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Buy);
    let order2 = order_helper(OrderID(1), Price(3), Side::Buy);
    let order3 = order_helper(OrderID(2), Price(2), Side::Buy);

    order_book.insert(order1);
    order_book.insert(order2);
    order_book.insert(order3);

    assert_eq!(order_book.best_bid(), OrderID(1));
    assert_eq!(order_book.bid_count(), 3);
    assert_eq!(order_book.ask_count(), 0);
    assert_eq!(order_book.price_level_count(), 2);
}

fn construct_ask_datastruct() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Sell);
    let order2 = order_helper(OrderID(1), Price(3), Side::Sell);
    let order3 = order_helper(OrderID(2), Price(2), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);
    order_book.insert(order3);

    assert_eq!(order_book.best_ask(), OrderID(0));
    assert_eq!(order_book.ask_count(), 3);
    assert_eq!(order_book.bid_count(), 0);
    assert_eq!(price_level_count(), 2);
}

#[test]
fn orderid_sort_in_btree() {
    let mut pricelvl = PriceLevel::new();

    for i in (0..2).rev() {
        for j in 0..3 {
            let order = order_helper(OrderID(i), Price(j), Side::Buy);
            pricelvl.insert(order.id, order);
        }
    }

    let collected_keys: Vec<&OrderID> = pricelvl.orders.keys().collect();
    let expected_keys: Vec<&OrderID> = vec![
        &OrderID(0),
        &OrderID(1),
        &OrderID(2),
        &OrderID(3),
        &OrderID(4),
        &OrderID(5),
    ];

    println!("{:?}", pricelvl.orders);

    assert_eq!(collected_keys, expected_keys);
}

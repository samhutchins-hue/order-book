use crate::Price;
use crate::*;

fn order_helper(id: OrderID, price: Price, side: Side) -> LimitOrder {
    LimitOrder {
        id,
        quantity: Quantity::new(1).unwrap(),
        price,
        side,
    }
}

#[test]
fn best_bid_is_highest_price() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Buy);
    let order2 = order_helper(OrderID(1), Price(3), Side::Buy);
    let order3 = order_helper(OrderID(2), Price(2), Side::Buy);

    order_book.insert(order1);
    order_book.insert(order2);
    order_book.insert(order3);

    assert_eq!(order_book.best_bid(), Some(Price(3)), "best_bid failed");
    assert_eq!(order_book.bid_count(), 3, "bid_count failed");
    assert_eq!(order_book.ask_count(), 0, "ask_count failed");
    assert_eq!(order_book.bid_level_count(), 2, "bid_level_count failed");
}

#[test]
fn construct_ask_datastruct() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Sell);
    let order2 = order_helper(OrderID(1), Price(3), Side::Sell);
    let order3 = order_helper(OrderID(2), Price(2), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);
    order_book.insert(order3);

    assert_eq!(order_book.best_ask(), Some(Price(2)));
    assert_eq!(order_book.ask_count(), 3);
    assert_eq!(order_book.bid_count(), 0);
    assert_eq!(order_book.ask_level_count(), 2);
}

#[test]
fn cancel_order_bid_test() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Buy);

    order_book.insert(order1);

    assert_eq!(order_book.total_orders(), 1);

    let cancel_res = order_book.cancel_order(OrderID(0));
    assert_eq!(cancel_res.unwrap().id, OrderID(0));
    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.bid_level_count(), 0);
}

#[test]
fn cancel_order_ask_test() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(2), Side::Sell);

    order_book.insert(order1);

    assert_eq!(order_book.total_orders(), 1);

    let cancel_res = order_book.cancel_order(OrderID(0));
    assert_eq!(cancel_res.unwrap().id, OrderID(0));
    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.bid_level_count(), 0);
}
#[test]
fn process_market_order_buy_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = order_helper(OrderID(0), Price(10), Side::Sell);
    let order2 = order_helper(OrderID(1), Price(20), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = MarketOrder {
        id: OrderID(2),
        quantity: Quantity::new(2).unwrap(),
        side: Side::Buy,
    };

    let result = order_book.process_market_order(order3);

    assert_eq!(order_book.total_orders(), 0);
}

#[test]
fn process_market_order_buy_partial_test() {
    todo!()
}
#[test]
fn process_limit_order_buy_fill_test() {
    todo!()
}

#[test]
fn process_limit_order_buy_partial_test() {
    todo!()
}

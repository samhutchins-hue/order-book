use crate::Price;
use crate::*;

fn limit_order_helper(id: OrderID, price: Price, side: Side) -> LimitOrder {
    LimitOrder::new(id, Quantity::new(1).unwrap(), price, side)
}

fn market_order_helper(id: u32, quantity: u64, side: Side) -> MarketOrder {
    MarketOrder::new(OrderID(id), Quantity::new(quantity).unwrap(), side)
}

#[test]
fn best_bid_is_highest_price() {
    let mut order_book = OrderBook::new();

    let order1 = limit_order_helper(OrderID(0), Price(2), Side::Buy);
    let order2 = limit_order_helper(OrderID(1), Price(3), Side::Buy);
    let order3 = limit_order_helper(OrderID(2), Price(2), Side::Buy);

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

    let order1 = limit_order_helper(OrderID(0), Price(2), Side::Sell);
    let order2 = limit_order_helper(OrderID(1), Price(3), Side::Sell);
    let order3 = limit_order_helper(OrderID(2), Price(2), Side::Sell);

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

    let order1 = limit_order_helper(OrderID(0), Price(2), Side::Buy);

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

    let order1 = limit_order_helper(OrderID(0), Price(2), Side::Sell);

    order_book.insert(order1);

    assert_eq!(order_book.total_orders(), 1);

    let cancel_res = order_book.cancel_order(OrderID(0));

    assert_eq!(cancel_res.unwrap().id, OrderID(0));
    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.bid_level_count(), 0);
}

#[test]
fn market_order_buy_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = limit_order_helper(OrderID(0), Price(10), Side::Sell);
    let order2 = limit_order_helper(OrderID(1), Price(20), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = MarketOrder {
        id: OrderID(2),
        quantity: Quantity::new(2).unwrap(),
        side: Side::Buy,
    };

    let result = order_book.process_order(Order::Market(order3));

    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.ask_level_count(), 0);
    assert_eq!(result.status, OrderStatus::Filled);
    let expected_fills = vec![
        Fill {
            maker_id: OrderID(0),
            taker_id: OrderID(2),
            quantity: Quantity(1),
            price: Price(10),
        },
        Fill {
            maker_id: OrderID(1),
            taker_id: OrderID(2),
            quantity: Quantity(1),
            price: Price(20),
        },
    ];
    assert_eq!(&result.fills, &expected_fills);
}

#[test]
fn market_order_buy_partial_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Sell);
    // let order2 = LimitOrder::new(OrderID(0), Quantity(10), Price(20), Side::Sell);

    order_book.insert(order1);
    // order_book.insert(order2);

    let order3 = market_order_helper(2, 10, Side::Buy);

    let result = order_book.process_order(Order::Market(order3));

    let expected_fills = vec![Fill {
        maker_id: OrderID(0),
        taker_id: OrderID(2),
        quantity: Quantity(5),
        price: Price(10),
    }];
    let expected_status: OrderStatus = OrderStatus::Canceled(Quantity(5));

    assert_eq!(result.fills, expected_fills);
    assert_eq!(result.status, expected_status);
    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.ask_level_count(), 0);
}

#[test]
fn market_order_empties_one_level() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Sell);
    let order2 = LimitOrder::new(OrderID(1), Quantity(10), Price(20), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = market_order_helper(2, 10, Side::Buy);

    let result = order_book.process_order(Order::Market(order3));

    let expected_fills = vec![
        Fill {
            maker_id: OrderID(0),
            taker_id: OrderID(2),
            quantity: Quantity(5),
            price: Price(10),
        },
        Fill {
            maker_id: OrderID(1),
            taker_id: OrderID(2),
            quantity: Quantity(5),
            price: Price(20),
        },
    ];

    let expected_status = OrderStatus::Filled;

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
    assert_eq!(order_book.total_orders(), 1);
    assert_eq!(order_book.ask_level_count(), 1);
}

#[test]
fn market_order_sell_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Buy);
    let order2 = LimitOrder::new(OrderID(1), Quantity(10), Price(20), Side::Buy);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = market_order_helper(2, 10, Side::Sell);

    let result = order_book.process_order(Order::Market(order3));

    assert_eq!(order_book.total_orders(), 1);
    assert_eq!(order_book.bid_level_count(), 1);

    let expected_fills = vec![Fill {
        maker_id: OrderID(1),
        taker_id: OrderID(2),
        quantity: Quantity(10),
        price: Price(20),
    }];

    let expected_status = OrderStatus::Filled;

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
}

#[test]
fn process_limit_order_buy_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Sell);
    let order2 = LimitOrder::new(OrderID(1), Quantity(5), Price(20), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = LimitOrder::new(OrderID(2), Quantity(6), Price(20), Side::Buy);

    let result = order_book.process_order(Order::Limit(order3));

    assert_eq!(order_book.total_orders(), 1);
    assert_eq!(order_book.ask_count(), 1);

    let expected_fills = vec![
        Fill {
            maker_id: OrderID(0),
            taker_id: OrderID(2),
            quantity: Quantity(5),
            price: Price(10),
        },
        Fill {
            maker_id: OrderID(1),
            taker_id: OrderID(2),
            quantity: Quantity(1),
            price: Price(20),
        },
    ];

    let expected_status = OrderStatus::Filled;
    assert_eq!(&result.status, &expected_status);
    assert_eq!(&result.fills, &expected_fills);
}

#[test]
fn process_limit_order_sell_fill_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Buy);
    let order2 = LimitOrder::new(OrderID(1), Quantity(5), Price(20), Side::Buy);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = LimitOrder::new(OrderID(2), Quantity(10), Price(5), Side::Sell);

    let result = order_book.process_order(Order::Limit(order3));

    assert_eq!(order_book.total_orders(), 0);
    assert_eq!(order_book.bid_level_count(), 0);

    let expected_fills = vec![
        Fill {
            maker_id: OrderID(1),
            taker_id: OrderID(2),
            quantity: Quantity(5),
            price: Price(20),
        },
        Fill {
            maker_id: OrderID(0),
            taker_id: OrderID(2),
            quantity: Quantity(5),
            price: Price(10),
        },
    ];

    let expected_status = OrderStatus::Filled;

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
}

#[test]
fn process_limit_order_buy_crossover_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Sell);
    let order2 = LimitOrder::new(OrderID(1), Quantity(5), Price(100), Side::Sell);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = LimitOrder::new(OrderID(2), Quantity(6), Price(50), Side::Buy);

    dbg!(&order_book.orders, "orders");
    dbg!(&order_book.bids, "bids");
    dbg!(&order_book.asks, "asks");

    let result = order_book.process_order(Order::Limit(order3));

    dbg!(&order_book.orders, "orders");
    dbg!(&order_book.bids, "bids");
    dbg!(&order_book.asks, "asks");

    assert_eq!(order_book.total_orders(), 2);
    assert_eq!(order_book.bid_level_count(), 1);
    assert_eq!(order_book.ask_level_count(), 1);

    let expected_fills = vec![Fill {
        maker_id: OrderID(0),
        taker_id: OrderID(2),
        quantity: Quantity(5),
        price: Price(10),
    }];

    let expected_status = OrderStatus::Resting(Quantity(1));

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
}

#[test]
fn process_limit_order_sell_crossover_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Buy);
    let order2 = LimitOrder::new(OrderID(1), Quantity(5), Price(100), Side::Buy);

    order_book.insert(order1);
    order_book.insert(order2);

    let order3 = LimitOrder::new(OrderID(2), Quantity(6), Price(50), Side::Sell);

    let result = order_book.process_order(Order::Limit(order3));

    assert_eq!(order_book.total_orders(), 2);
    assert_eq!(order_book.bid_level_count(), 1);
    assert_eq!(order_book.ask_level_count(), 1);

    let expected_fills = vec![Fill {
        maker_id: OrderID(1),
        taker_id: OrderID(2),
        quantity: Quantity(5),
        price: Price(100),
    }];

    let expected_status = OrderStatus::Resting(Quantity(1));

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
}

#[test]
fn process_limit_order_buy_partial_test() {
    let mut order_book = OrderBook::new();

    let order1 = LimitOrder::new(OrderID(0), Quantity(5), Price(10), Side::Buy);

    order_book.insert(order1);

    let order3 = LimitOrder::new(OrderID(2), Quantity(10), Price(10), Side::Sell);

    let result = order_book.process_order(Order::Limit(order3));

    assert_eq!(order_book.total_orders(), 1);
    assert_eq!(order_book.bid_level_count(), 0);
    assert_eq!(order_book.ask_level_count(), 1);

    let expected_fills = vec![Fill {
        maker_id: OrderID(0),
        taker_id: OrderID(2),
        quantity: Quantity(5),
        price: Price(10),
    }];

    let expected_status = OrderStatus::Resting(Quantity(5));

    assert_eq!(&result.fills, &expected_fills);
    assert_eq!(&result.status, &expected_status);
}

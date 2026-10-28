#[cfg(test)]
mod test;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// positive, monotonically increasing
// to allow ordering of FIFO queue of requests

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct OrderID {
    id: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Quantity {
    quantity: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Price {
    price: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum OrderType {
    Buy,
    Sell,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct MarketOrder {
    id: OrderID,
    quantity: Quantity,
    side: OrderType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct LimitOrder {
    id: OrderID,
    quantity: Quantity,
    price: Price,
    side: OrderType,
}

struct OrderBook {
    lowest_bid_id: OrderID,
    lowets_bid_price: Price,
    lowest_ask_id: OrderID,
    lowest_ask_price: Price,
    bids: BTreeMap<Price, BTreeSet<Order>>,
    asks: BTreeMap<Price, BTreeSet<Order>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m_order = MarketOrder {
            id: OrderID { id: 0 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        };
    }
}

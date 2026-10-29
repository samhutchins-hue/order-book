#[cfg(test)]
mod test;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

// positive, monotonically increasing
// to allow ordering of FIFO queue of requests

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct OrderID(u64);

#[derive(Copy, Clone, Debug)]
struct Quantity(u64);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Price(u64);

#[derive(Debug)]
enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Debug)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug)]
struct MarketOrder {
    id: OrderID,
    quantity: Quantity,
    side: OrderType,
}

#[derive(Debug)]
struct LimitOrder {
    id: OrderID,
    quantity: Quantity,
    price: Price,
    side: OrderType,
}

struct PriceLevel {
    orders: BTreeMap<OrderID, Order>,
    running_total: u64,
}

struct OrderBook {
    bids: BTreeMap<Price, PriceLevel>,
    asks: BTreeMap<Price, PriceLevel>,
    orders: HashMap<OrderID, (OrderType, Price)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m_order = MarketOrder {
            id: OrderID(0),
            quantity: Quantity(5),
            side: OrderType::Buy,
        };
    }
}

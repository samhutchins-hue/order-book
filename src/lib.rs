#[cfg(test)]
mod test;
use std::collections::BTreeMap;
use std::collections::HashMap;

// positive, monotonically increasing
// to allow ordering of FIFO queue of requests
#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct OrderID(u32);

#[derive(Copy, Clone, Debug, Default)]
struct Quantity(u64);

impl Quantity {
    fn new(quantity: u64) -> Option<Self> {
        if quantity == 0 {
            return None;
        }
        Some(Quantity(quantity))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Price(u64);

#[derive(Debug)]
enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Copy, Clone, Debug)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
struct MarketOrder {
    id: OrderID,
    quantity: Quantity,
    side: Side,
}

#[derive(Debug)]
struct LimitOrder {
    id: OrderID,
    quantity: Quantity,
    price: Price,
    side: Side,
}

struct PriceLevel {
    orders: BTreeMap<OrderID, LimitOrder>,
    total_count: u64,
}

pub struct OrderBook {
    bids: BTreeMap<Price, PriceLevel>,
    asks: BTreeMap<Price, PriceLevel>,
    orders: HashMap<OrderID, (Side, Price)>,
    next_id: u32,
}

impl PriceLevel {
    fn new() -> Self {
        Self {
            orders: BTreeMap::new(),
            total_count: 0,
        }
    }

    fn insert(&mut self, order_id: OrderID, order: LimitOrder) {
        if let Some(existing) = self.orders.insert(order_id, order) {
            panic!("duplicate order_id found {:?}", existing);
        };
        self.total_count += 1;
    }

    fn total(&self) -> u64 {
        return self.total_count;
    }

    fn remove(&mut self, order_id: OrderID) {
        if let Some(order) = self.orders.remove_entry(&order_id) {
            panic!("order with id: {:?} not found", order);
        };
    }
}

impl LimitOrder {
    fn new(id: OrderID, quantity: Quantity, price: Price, side: Side) -> Self {
        Self {
            id,
            quantity,
            price,
            side,
        }
    }
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: HashMap::new(),
            next_id: 0,
        }
    }

    fn process_order(&mut self, order: Order) {
        match order {
            Order::Market(market_order) => todo!(),
            Order::Limit(limit_order) => todo!(),
        }
    }

    fn insert(&mut self, order: LimitOrder) {
        // insert into the hashmap
        if let Some(existing) = self.orders.insert(order.id, (order.side, order.price)) {
            panic!("duplicate order_id found {:?}", existing);
        };

        // insert into either bids or asks
        match order.side {
            Side::Buy => self
                .bids
                .entry(order.price)
                .or_insert_with(PriceLevel::new)
                .insert(order.id, order),
            Side::Sell => self
                .asks
                .entry(order.price)
                .or_insert_with(PriceLevel::new)
                .insert(order.id, order),
        }
    }
    fn cancel_order(&mut self, order_id: OrderID) {
        todo!()
    }
    fn total_orders(&self) -> u64 {
        todo!()
    }

    fn best_bid(&self) -> OrderID {
        todo!()
    }

    fn bid_count(&self) -> u64 {
        todo!()
    }
    fn ask_count(&self) -> u64 {
        todo!()
    }
    fn price_level_count(&self) -> u64 {
        todo!()
    }

    fn best_ask(&self) -> OrderID {
        todo!()
    }
}

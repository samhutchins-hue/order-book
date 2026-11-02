#[cfg(test)]
mod test;
use std::collections::BTreeMap;
use std::collections::HashMap;

// positive, monotonically increasing
// to allow ordering of FIFO queue of requests
#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct OrderID(u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Quantity(u64);

impl Quantity {
    fn new(quantity: u64) -> Option<Self> {
        if quantity == 0 {
            return None;
        }
        Some(Quantity(quantity))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Price(u64);

#[derive(Debug)]
enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(Debug, PartialEq)]
struct LimitOrder {
    id: OrderID,
    quantity: Quantity,
    price: Price,
    side: Side,
}

#[derive(Debug, PartialEq, Eq)]
enum CancelError {
    UnknownOrder(OrderID),
}

struct PriceLevel {
    orders: BTreeMap<OrderID, LimitOrder>,
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
        }
    }

    fn insert(&mut self, order: LimitOrder) {
        if let Some(existing) = self.orders.insert(order.id, order) {
            panic!("duplicate order_id found {:?}", existing);
        };
    }

    fn order_count(&self) -> usize {
        self.orders.len()
    }

    fn remove(&mut self, order_id: OrderID) -> LimitOrder {
        match self.orders.remove_entry(&order_id) {
            Some(kv) => kv.1,
            None => {
                panic!("order with id: {:?} not found", order_id);
            }
        }
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

    fn process_limit_order(&mut self, order: LimitOrder) {
        match order.side {
            Side::Buy => todo!(),
            Side::Sell => todo!(),
        }
    }

    fn process_market_order(&mut self, order: MarketOrder) {
        match order.side {
            Side::Buy => todo!(),
            Side::Sell => todo!(),
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
                .insert(order),
            Side::Sell => self
                .asks
                .entry(order.price)
                .or_insert_with(PriceLevel::new)
                .insert(order),
        }
    }

    fn cancel_order(&mut self, order_id: OrderID) -> Result<LimitOrder, CancelError> {
        let (order_side, order_price) = self
            .orders
            .remove(&order_id)
            .ok_or(CancelError::UnknownOrder(order_id))?;

        match order_side {
            // go to price level provided by hashmap lookup (gives side and price)
            // call remove on that price level
            Side::Buy => {
                let Some(price_level) = self.bids.get_mut(&order_price) else {
                    return Err(CancelError::UnknownOrder(order_id));
                };
                let canceled_order = price_level.remove(order_id);
                if price_level.order_count() == 0 {
                    self.bids
                        .remove(&order_price)
                        .expect("expected price level to exist for removal");
                }
                Ok(canceled_order)
            }
            Side::Sell => {
                let Some(price_level) = self.asks.get_mut(&order_price) else {
                    return Err(CancelError::UnknownOrder(order_id));
                };
                let canceled_order = price_level.remove(order_id);
                if price_level.order_count() == 0 {
                    self.asks
                        .remove(&order_price)
                        .expect("expected price level to exist for removal");
                }
                Ok(canceled_order)
            }
        }
    }
    fn total_orders(&self) -> usize {
        self.bid_count() + self.ask_count()
    }

    fn best_bid(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(k, _v)| *k)
    }

    fn bid_count(&self) -> usize {
        self.bids
            .values()
            .map(|price_level| price_level.order_count())
            .sum()
    }

    fn ask_count(&self) -> usize {
        self.asks
            .values()
            .map(|price_level| price_level.order_count())
            .sum()
    }

    fn bid_level_count(&self) -> usize {
        self.bids.len()
    }

    fn ask_level_count(&self) -> usize {
        self.asks.len()
    }

    fn best_ask(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(k, _v)| *k)
    }
}

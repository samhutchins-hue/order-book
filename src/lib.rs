#[cfg(test)]
mod test;
use std::collections::BTreeMap;
use std::collections::HashMap;

// positive, monotonically increasing
// to allow ordering of FIFO queue of requests
#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct OrderID(u32);

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
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
struct Fill {
    maker_id: OrderID,
    taker_id: OrderID,
    quantity: Quantity,
    price: Price,
    // aggressor: Side,
}

#[derive(PartialEq, Eq, Debug)]
enum OrderStatus {
    Filled,
    Resting(Quantity),
    Canceled(Quantity),
}

struct MatchResult {
    fills: Vec<Fill>,
    status: OrderStatus,
}

impl MatchResult {
    fn new() -> Self {
        Self {
            fills: Vec::new(),
            status: OrderStatus::Filled,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CancelError {
    UnknownOrder(OrderID),
}

#[derive(Debug)]
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

impl MarketOrder {
    fn new(id: OrderID, quantity: Quantity, side: Side) -> Self {
        Self { id, quantity, side }
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

    // need the takers_id, side, quantity, Option<price>
    fn process_order(&mut self, order: Order) -> MatchResult {
        match order {
            Order::Market(market_order) => {
                let (fills, remaining) = self.match_order_helper(
                    market_order.id,
                    market_order.quantity,
                    market_order.side,
                    None,
                );
                if let Some(quantity) = remaining {
                    return MatchResult {
                        fills,
                        status: OrderStatus::Canceled(quantity),
                    };
                }
                MatchResult {
                    fills,
                    status: OrderStatus::Filled,
                }
            }
            Order::Limit(limit_order) => {
                let (fills, remaining) = self.match_order_helper(
                    limit_order.id,
                    limit_order.quantity,
                    limit_order.side,
                    Some(limit_order.price),
                );
                if let Some(quantity) = remaining {
                    self.rest_limit_order(limit_order, quantity);
                    return MatchResult {
                        fills,
                        status: OrderStatus::Resting(quantity),
                    };
                }
                MatchResult {
                    fills,
                    status: OrderStatus::Filled,
                }
            }
        }
    }

    fn match_order_helper(
        &mut self,
        taker_id: OrderID,
        quantity: Quantity,
        side: Side,
        maybe_price_limit: Option<Price>,
    ) -> (Vec<Fill>, Option<Quantity>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut remaining = quantity.0;

        let book = match side {
            Side::Buy => &mut self.asks,
            Side::Sell => &mut self.bids,
        };

        'level_loop: loop {
            let best_level = match side {
                Side::Buy => book.first_entry(),
                Side::Sell => book.last_entry(),
            };

            let Some(mut level_entry) = best_level else {
                break 'level_loop;
            };

            // copy out price and get mutable reference to the current level for potential removal
            let level_price = *level_entry.key();
            let level = level_entry.get_mut();

            // match price {
            //     Some(price) => {
            //         if side == Side::Buy && *level_entry.key() > price {
            //             break 'level_loop;
            //         }
            //         if side == Side::Sell && *level_entry.key() < price {
            //             break 'level_loop;
            //         }
            //     }
            //     None => (),
            // }

            if let Some(limit) = maybe_price_limit {
                let crossover_limit = match side {
                    Side::Buy => level_price > limit,
                    Side::Sell => level_price < limit,
                };
                if crossover_limit {
                    break 'level_loop;
                }
            }

            'order_loop: loop {
                let Some(mut order_entry) = level.orders.first_entry() else {
                    break 'order_loop;
                };

                let maker = order_entry.get_mut();

                let fill_qty = std::cmp::min(maker.quantity.0, remaining);

                let fill_record = Fill {
                    maker_id: maker.id,
                    taker_id,
                    quantity: Quantity(fill_qty),
                    price: maker.price,
                };
                fills.push(fill_record);

                remaining -= fill_qty;

                if fill_qty == maker.quantity.0 {
                    let filled_maker = order_entry.remove();
                    self.orders.remove(&filled_maker.id);
                } else {
                    maker.quantity.0 -= fill_qty;
                }

                if remaining == 0 {
                    break 'order_loop;
                }
            }

            if level.orders.is_empty() {
                level_entry.remove();
            }

            if remaining == 0 {
                break 'level_loop;
            }
        }

        (fills, Some(Quantity(remaining)))
    }

    fn rest_limit_order(&mut self, mut order: LimitOrder, new_quantity: Quantity) {
        order.quantity = new_quantity;
        self.insert(order);
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

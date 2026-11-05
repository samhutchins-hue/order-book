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

    fn process_order(&mut self, order: Order) {
        match order {
            Order::Market(market_order) => todo!(),
            Order::Limit(limit_order) => todo!(),
        }
    }

    fn process_limit_order(&mut self, order: LimitOrder) -> MatchResult {
        match order.side {
            Side::Buy => self.match_limit_order(&order),
            Side::Sell => self.match_limit_order(&order),
        }
    }

    fn match_limit_order(&mut self, order: &LimitOrder) -> MatchResult {
        let mut match_result = MatchResult::new();
        let mut remaining = order.quantity.0;

        let book = if order.side == Side::Buy {
            &mut self.asks
        } else {
            &mut self.bids
        };

        'level_loop: loop {
            let best_level = match order.side {
                Side::Buy => book.first_entry(),
                Side::Sell => book.last_entry(),
            };

            let Some(mut level_entry) = best_level else {
                break 'level_loop;
            };

            if order.side == Side::Buy && *level_entry.key() > order.price {
                break 'level_loop;
            }
            if order.side == Side::Sell && *level_entry.key() < order.price {
                break 'level_loop;
            }

            let level = level_entry.get_mut();

            'order_loop: loop {
                let Some(mut order_entry) = level.orders.first_entry() else {
                    break 'order_loop;
                };

                let maker = order_entry.get_mut();

                let fill_qty = std::cmp::min(maker.quantity.0, remaining);

                let fill_record = Fill {
                    maker_id: maker.id,
                    taker_id: order.id,
                    quantity: Quantity(fill_qty),
                    price: maker.price,
                };

                match_result.fills.push(fill_record);

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

        if remaining != 0 {
            match_result.status = OrderStatus::Resting(Quantity(remaining));
        } else {
            match_result.status = OrderStatus::Filled;
        }

        match_result
    }

    fn match_market_order(&mut self, order: &MarketOrder) -> MatchResult {
        let mut match_result = MatchResult::new();
        let mut remaining = order.quantity.0;

        let book = if order.side == Side::Buy {
            &mut self.asks
        } else {
            &mut self.bids
        };

        'level_loop: loop {
            // get the best (price, price level) of ask for a buy
            let best_level = match order.side {
                Side::Buy => book.first_entry(),
                Side::Sell => book.last_entry(),
            };
            let Some(mut level_entry) = best_level else {
                // report that there is no asks level so cancel remaining order
                break 'level_loop;
            };
            // let best_price = *level_entry.key();
            let level = level_entry.get_mut();

            'order_loop: loop {
                // get oldest order at price level we want to buy from
                let Some(mut order_entry) = level.orders.first_entry() else {
                    break 'order_loop;
                };

                let maker = order_entry.get_mut();

                let fill_qty = std::cmp::min(maker.quantity.0, remaining);

                let fill_record = Fill {
                    maker_id: maker.id,
                    taker_id: order.id,
                    quantity: Quantity(fill_qty),
                    price: maker.price,
                };
                match_result.fills.push(fill_record);

                remaining -= fill_qty;

                if fill_qty == maker.quantity.0 {
                    // decrement required quantity from current order and remove maker from price level
                    let filled_maker = order_entry.remove();
                    self.orders.remove(&filled_maker.id);
                } else {
                    // subtract required quantity from maker
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

        if remaining != 0 {
            match_result.status = OrderStatus::Canceled(Quantity(remaining));
        } else {
            match_result.status = OrderStatus::Filled;
        }

        match_result
    }

    fn process_market_order(&mut self, order: MarketOrder) -> MatchResult {
        // trying to fill the market order's quantity
        // by getting the best ask and seeing if it can fill the market order completely
        // current best_ask can fill perfectly, fill order partially
        // when an order uses a best ask it can either take quanity of it partially, or completely.
        //
        // so we need to update the state of the current market order and the best_ask
        // market_order: partially filled, fully filled
        // best_ask: partially filled, fully filled (remove from pricelevel)
        //
        // so iterate over the pricelevels
        // iterate over each order in price level until market order is filled: {
        //
        //      1. if order filled, break loop
        //
        //      2. if maker order filled, remove maker from pricelevel and check if price level is empty
        //      and rmeove if so
        //
        //  }

        self.match_market_order(&order)
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

use crate::MarketOrder;
use crate::Order;
use crate::Price;
use crate::*;
use std::collections::BTreeMap;
// use std::collections::BTreeSet;
use std::collections::VecDeque;

fn order_helper(id: OrderID, price: Price) -> LimitOrder {
    LimitOrder {
        id,
        quantity: Quantity(1),
        price,
        side: OrderType::Buy,
    }
}

#[test]
fn order_id_works() {
    let order_id = OrderID(0);
    assert_eq!(order_id.0, 0);
}

#[test]
fn btree_test() {
    let mut bt: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for i in (0..100).rev() {
        for j in 0..5 {
            bt.entry(i).or_default().insert(j);
        }
    }
    assert_eq!(bt.len(), 100);
    assert_eq!(
        *bt.last_key_value().unwrap().1,
        BTreeSet::from([0, 1, 2, 3, 4])
    );
    assert_eq!(bt.last_key_value().unwrap().0, &99);

    let expected = BTreeSet::from([0, 1, 2, 3, 4]);
    for set in bt.values() {
        assert_eq!(*set, expected);
    }

    println!("{:?}", bt);
}

#[test]
fn price_and_order_in_btree() {
    let mut bt: BTreeMap<Price, VecDeque<Order>> = BTreeMap::new();

    for i in (0..3).rev() {
        for j in 0..3 {
            let order = order_helper(OrderID(j), Price(i));
            bt.entry(order.price)
                .or_default()
                .push_back(Order::Limit(order));
        }
    }

    let collected_keys: Vec<&Price> = bt.keys().collect();
    let expected_keys: Vec<&Price> = vec![&Price(0), &Price(1), &Price(2)];
    assert_eq!(collected_keys, expected_keys);
}

#[test]
fn orderid_sort_in_btree() {
    let mut pricelvl: PriceLevel = PriceLevel {
        orders: BTreeMap::new(),
        running_total: 0,
    };

    for i in (0..3).rev() {
        for j in 0..3 {
            let order = order_helper(OrderID(j), Price(i));
            pricelvl.orders.insert(order.id, Order::Limit(order));
            pricelvl.running_total += 1;
        }
    }

    let collected_keys: Vec<&OrderID> = pricelvl.orders.keys().collect();
    let expected_keys: Vec<&OrderID> = vec![&OrderID(0), &OrderID(1), &OrderID(2)];

    assert_eq!(collected_keys, expected_keys);
}

use crate::MarketOrder;
use crate::Order;
use crate::Price;
use crate::*;
use std::collections::BTreeMap;
// use std::collections::BTreeSet;
use std::collections::VecDeque;

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
    for (_, set) in &bt {
        assert_eq!(*set, expected);
    }

    println!("{:?}", bt);
}

#[test]
fn price_and_order_in_btree() {
    let mut bt: BTreeMap<Price, VecDeque<Order>> = BTreeMap::new();

    for i in (0..100) {
        for j in 0..5 {
            bt.entry(Price { price: i })
                .or_default()
                .push_back(Order::Market(MarketOrder {
                    id: OrderID { id: j },
                    quantity: Quantity { quantity: 5 },
                    side: OrderType::Buy,
                }));
        }
    }

    let expected = BTreeSet::from([
        Order::Market(MarketOrder {
            id: OrderID { id: 0 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        }),
        Order::Market(MarketOrder {
            id: OrderID { id: 1 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        }),
        Order::Market(MarketOrder {
            id: OrderID { id: 2 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        }),
        Order::Market(MarketOrder {
            id: OrderID { id: 3 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        }),
        Order::Market(MarketOrder {
            id: OrderID { id: 4 },
            quantity: Quantity { quantity: 5 },
            side: OrderType::Buy,
        }),
    ]);

    for (_, set) in &bt {
        assert_eq!(*set, expected);
    }

    assert_eq!(*bt.last_key_value().unwrap().0, Price { price: 99 });
}

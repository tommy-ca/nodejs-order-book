use std::collections::BTreeMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ordered_float::OrderedFloat;

#[derive(Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct Order {
    pub id: u32,
    pub price: f64,
    pub qty: f64,
}

#[derive(Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct Trade {
    pub maker_id: u32,
    pub taker_id: u32,
    pub price: f64,
    pub qty: f64,
}

struct Book {
    bids: BTreeMap<OrderedFloat<f64>, Vec<Order>>, // descending by price
    asks: BTreeMap<OrderedFloat<f64>, Vec<Order>>, // ascending by price
}

static BOOK: Lazy<Mutex<Book>> = Lazy::new(|| {
    Mutex::new(Book {
        bids: BTreeMap::new(),
        asks: BTreeMap::new(),
    })
});

fn match_order(side: &str, incoming: &Order, book: &mut Book) -> Option<Trade> {
    match side {
        "bid" => {
            if let Some((&best_price, orders)) = book.asks.iter_mut().next() {
                if incoming.price >= best_price.into_inner() {
                    if let Some(maker) = orders.first_mut() {
                        let qty = incoming.qty.min(maker.qty);
                        let trade = Trade {
                            maker_id: maker.id,
                            taker_id: incoming.id,
                            price: best_price.into_inner(),
                            qty,
                        };
                        maker.qty -= qty;
                        if maker.qty <= 0.0 {
                            orders.remove(0);
                        }
                        if orders.is_empty() {
                            book.asks.remove(&best_price);
                        }
                        return Some(trade);
                    }
                }
            }
            book
                .bids
                .entry(OrderedFloat(incoming.price))
                .or_default()
                .push(incoming.clone());
            None
        }
        "ask" => {
            if let Some((&best_price, orders)) = book.bids.iter_mut().rev().next() {
                if incoming.price <= best_price.into_inner() {
                    if let Some(maker) = orders.first_mut() {
                        let qty = incoming.qty.min(maker.qty);
                        let trade = Trade {
                            maker_id: maker.id,
                            taker_id: incoming.id,
                            price: best_price.into_inner(),
                            qty,
                        };
                        maker.qty -= qty;
                        if maker.qty <= 0.0 {
                            orders.remove(0);
                        }
                        if orders.is_empty() {
                            book.bids.remove(&best_price);
                        }
                        return Some(trade);
                    }
                }
            }
            book
                .asks
                .entry(OrderedFloat(incoming.price))
                .or_default()
                .push(incoming.clone());
            None
        }
        _ => None,
    }
}

#[napi]
pub fn place_limit(side: String, order: Order) -> Option<Trade> {
    let mut book = BOOK.lock().unwrap();
    match_order(&side, &order, &mut book)
}

#[napi]
pub fn clear_book() {
    let mut book = BOOK.lock().unwrap();
    book.bids.clear();
    book.asks.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_trade() {
        clear_book();
        let order1 = Order { id: 1, price: 100.0, qty: 1.0 };
        let order2 = Order { id: 2, price: 100.0, qty: 1.0 };
        let _ = place_limit("ask".to_string(), order1);
        let trade = place_limit("bid".to_string(), order2).expect("trade");
        assert_eq!(trade.price, 100.0);
        assert_eq!(trade.qty, 1.0);
    }
}


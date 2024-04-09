mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_rafli = Order::new(BidOrAsk::Bid, 2.5);
    // let sell_order = Order::new(BidOrAsk::Ask, 2.45);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_rafli);

    let sell_order_from_heaven = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(6.5, sell_order_from_heaven);
    println!("{:?}", orderbook);
}

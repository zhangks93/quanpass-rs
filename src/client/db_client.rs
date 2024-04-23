use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

struct DBClient {
    pub conn: Connection,
}

impl DBClient {
    fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        DBClient { conn }
    }

    fn get_instance() -> &'static Mutex<DBClient> {
        lazy_static! {
            static ref INSTANCE: Mutex<DBClient> = Mutex::new(DBClient::new());
        }
        &INSTANCE
    }

    // Example method to create a table
    pub fn create_order_table(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS orders (
                symbol TEXT NOT NULL,
                order_id INTEGER PRIMARY KEY,
                order_list_id INTEGER,
                client_order_id TEXT NOT NULL,
                price REAL NOT NULL,
                orig_qty TEXT NOT NULL,
                executed_qty TEXT NOT NULL,
                cummulative_quote_qty TEXT NOT NULL,
                status TEXT NOT NULL,
                time_in_force TEXT NOT NULL,
                type_name TEXT NOT NULL,
                side TEXT NOT NULL,
                stop_price REAL NOT NULL,
                iceberg_qty TEXT NOT NULL,
                time INTEGER NOT NULL,
                update_time INTEGER NOT NULL,
                is_working INTEGER NOT NULL,
                orig_quote_order_qty TEXT NOT NULL
            );",
                [],
            )
            .unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::client::binance_domain::Order;

    use super::DBClient;

    #[test]
    fn test_limit_sell_and_limit_buy() {
        let db_client_guard = DBClient::get_instance()
            .lock()
            .unwrap()
            .create_order_table()
            .unwrap();

        /* db_client_guard.create_order_table().unwrap();
            let order = Order {
                symbol: "BTCUSDT".to_string(),
                order_id: 1u64,
                order_list_id: 0,
                client_order_id: "testClientOrderId".to_string(),
                price: 50000.0,
                orig_qty: "1.0".to_string(),
                executed_qty: "0.0".to_string(),
                cummulative_quote_qty: "0.0".to_string(),
                status: "NEW".to_string(),
                time_in_force: "GTC".to_string(),
                type_name: "LIMIT".to_string(),
                side: "BUY".to_string(),
                stop_price: 0.0,
                iceberg_qty: "0.0".to_string(),
                time: 1622549728473,
                update_time: 1622549728473,
                is_working: true,
                orig_quote_order_qty: "0.0".to_string(),
            };
            db_client.unwrap().conn.execute(
            "INSERT INTO orders (symbol, order_id, order_list_id, client_order_id, price, orig_qty, executed_qty, cummulative_quote_qty, status, time_in_force, type_name, side, stop_price, iceberg_qty, time, update_time, is_working, orig_quote_order_qty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                rusqlite::params![
                    order.symbol,
                    order.order_id,
                    order.order_list_id,
                    order.client_order_id,
                    order.price,
                    order.orig_qty,
                    order.executed_qty,
                    order.cummulative_quote_qty,
                    order.status,
                    order.time_in_force,
                    order.type_name,
                    order.side,
                    order.stop_price,
                    order.iceberg_qty,
                    order.time,
                    order.update_time,
                    order.is_working as i32, // Assuming is_working is a bool, SQLite uses integers for boolean values (0 for false, 1 for true)
                    order.orig_quote_order_qty,
                ],
        ).unwrap();

            let mut stmt = db_client.unwrap().conn.prepare("SELECT symbol, order_id, order_list_id, client_order_id, price, orig_qty, executed_qty, cummulative_quote_qty, status, time_in_force, type_name, side, stop_price, iceberg_qty, time, update_time, is_working, orig_quote_order_qty
        FROM orders").unwrap();
            let orders_iter = stmt
                .query_map([], |row| {
                    Ok(Order {
                        // Assuming you have a constructor or a way to create an Order from a row. Adjust the types as necessary.
                        symbol: row.get(0)?,
                        order_id: row.get(1)?,
                        order_list_id: row.get(2)?,
                        client_order_id: row.get(3)?,
                        price: row.get(4)?,
                        orig_qty: row.get(5)?,
                        executed_qty: row.get(6)?,
                        cummulative_quote_qty: row.get(7)?,
                        status: row.get(8)?,
                        time_in_force: row.get(9)?,
                        type_name: row.get(10)?,
                        side: row.get(11)?,
                        stop_price: row.get(12)?,
                        iceberg_qty: row.get(13)?,
                        time: row.get(14)?,
                        update_time: row.get(15)?,
                        is_working: row.get(16)?,
                        orig_quote_order_qty: row.get(17)?,
                    })
                })
                .unwrap(); */
    }
}

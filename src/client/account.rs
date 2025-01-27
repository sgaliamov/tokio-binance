use crate::builder::ParamBuilder;
use crate::client::*;
use crate::param::{OrderType, Parameters, Side, TimeInForce, ID};
use crate::types::*;
use reqwest::{Client, Url};

/// Client for dealing with orders
#[derive(Clone)]
pub struct AccountClient {
    api_key: String,
    secret_key: String,
    url: Url,
    client: Client,
}

impl AccountClient {
    /// Creates new client instance.
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{AccountClient, BINANCE_US_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect<A, S, U>(api_key: A, secret_key: S, url: U) -> crate::error::Result<Self>
    where
        A: Into<String>,
        S: Into<String>,
        U: Into<String>,
    {
        Ok(Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new(),
        })
    }
    /// Place a new limit order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::{Side::Sell, TimeInForce::Fok, OrderRespType::Full};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     // false will send as test, true will send as a real order.
    ///     .place_limit_order("BNBUSDT", Sell, 20.00, 5.00, false)
    ///     // optional: lifetime of order; default is Gtc.
    ///     .with_time_in_force(Fok)
    ///     // optional: unique id; auto generated by default.
    ///     .with_new_client_order_id("<uuid>")
    ///     // optional: splits quantity; sets time in force to Gtc.
    ///     .with_iceberg_qty(1.00)
    ///     // optional: output verbosity; default is Ack.
    ///     .with_new_order_resp_type(Full)
    ///     // optional: converts Limit to Stop-Limit; triggers when price hits below 21.00.
    ///     .with_stop_loss_limit(21.00)
    ///     // optional: converts Limit to Stop-Limit; triggers when price hits above 21.00.
    ///     .with_take_profit_limit(21.00)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     // optional: converts Limit to Limit-Maker; consumes builder and returns a different one.
    ///     .into_limit_maker_order()
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_limit_order<'a>(
        &self,
        symbol: &'a str,
        side: Side,
        price: f64,
        quantity: f64,
        execute: bool,
    ) -> ParamBuilder<'a, '_, LimitOrderParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = if execute {
            url.join("/api/v3/order").unwrap()
        } else {
            url.join("/api/v3/order/test").unwrap()
        };

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                side: Some(side),
                order_type: Some(OrderType::Limit),
                price: Some(price),
                quantity: Some(quantity),
                time_in_force: Some(TimeInForce::Gtc),
                ..Parameters::default()
            },
            client.post(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Place a new market order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::{Side::Sell, TimeInForce::Fok, OrderRespType::Full};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     // false will send as test, true will send as a real order.
    ///     .place_market_order("BNBUSDT", Sell, 5.00, false)
    ///     // optional: unique id; auto generated by default.
    ///     .with_new_client_order_id("<uuid>")
    ///     // optional: output verbosity; default is Ack.
    ///     .with_new_order_resp_type(Full)
    ///     // optional: converts Market to Stop-Loss; triggers when price hits below 21.00.
    ///     .with_stop_loss(21.00)
    ///     // optional: converts Market to Stop-Loss; triggers when price hits above 21.00.
    ///     .with_take_profit(21.00)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_market_order<'a>(
        &self,
        symbol: &'a str,
        side: Side,
        quantity: f64,
        execute: bool,
    ) -> ParamBuilder<'a, '_, MarketOrderParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = if execute {
            url.join("/api/v3/order").unwrap()
        } else {
            url.join("/api/v3/order/test").unwrap()
        };

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                side: Some(side),
                order_type: Some(OrderType::Market),
                quantity: Some(quantity),
                ..Parameters::default()
            },
            client.post(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::ID;
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_order("BNBUSDT", ID::ClientOId("<uuid>"))
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_order<'a>(
        &self,
        symbol: &'a str,
        id: ID<'a>,
    ) -> ParamBuilder<'a, '_, OrderStatusParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/order").unwrap();

        let order_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                order_id,
                orig_client_order_id,
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Cancel order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::ID;
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .cancel_order("BNBUSDT", ID::ClientOId("<uuid>"))
    ///     // optional: unique id; auto generated by default.
    ///     .with_new_client_order_id("<uuid>")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_order<'a>(
        &self,
        symbol: &'a str,
        id: ID<'a>,
    ) -> ParamBuilder<'a, '_, CancelOrderParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/order").unwrap();

        let order_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                order_id,
                orig_client_order_id,
                ..Parameters::default()
            },
            client.delete(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get open orders.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_open_orders()
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_open_orders(&self) -> ParamBuilder<'_, '_, OpenOrderParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/openOrders").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get all orders.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::hours(23);
    ///
    /// let response = client
    ///     .get_all_orders("BNBUSDT")
    ///     // optional: filter by orders greater than or equal to the provided id.
    ///     // If supplied, neither startTime or endTime can be provided
    ///     .with_order_id(1230494)
    ///     // optional: get orders from; pass 24 hours of orders is the default.
    ///     .with_start_time(start)
    ///     // optional: get orders until; default is now.
    ///     .with_end_time(end)
    ///     // optional: limit the amount of orders; default 500; max 1000.
    ///     .with_limit(100)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_all_orders<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, AllOrdersParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/allOrders").unwrap();

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Place a new oco order.
    /// # Price Restrictions:
    /// - SELL: Limit Price > Last Price > Stop Price
    /// - BUY: Limit Price < Last Price < Stop Price
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::{Side::Sell, TimeInForce::Gtc, OrderRespType::Full};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     // Limit to sell at 30.00 and Stop-Loss at 20.00; One cancels the other.
    ///     .place_oco_order("BNBUSDT", Sell, 30.00, 20.00, 5.00)
    ///     // optional: A unique Id for the entire orderList; auto generated by default.
    ///     .with_list_client_order_id("<uuid>")
    ///     // optional: A unique Id for the limit order; auto generated by default.
    ///     .with_limit_client_order_id("<uuid>")
    ///     // optional: splits quantity for the limit order leg;
    ///     .with_limit_iceberg_qty(1.00)
    ///     // optional: A unique Id for the stop loss/stop loss limit leg; auto generated by default.
    ///     .with_stop_client_order_id("<uuid>")
    ///     // optional: Converts Stop-Loss to Stop-Limit; triggers bellow 20.00.
    ///     .with_stop_limit_price(19.00, Gtc)
    ///     // optional: splits quantity for the stop order leg;
    ///     .with_stop_iceberg_qty(1.00)
    ///     // optional: output verbosity; default is Ack.
    ///     .with_new_order_resp_type(Full)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_oco_order<'a>(
        &self,
        symbol: &'a str,
        side: Side,
        price: f64,
        stop_price: f64,
        quantity: f64,
    ) -> ParamBuilder<'a, '_, OcoParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/order/oco").unwrap();

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                side: Some(side),
                price: Some(price),
                stop_price: Some(stop_price),
                quantity: Some(quantity),
                ..Parameters::default()
            },
            client.post(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Cancel oco order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::ID;
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .cancel_oco_order("BNBUSDT", ID::ClientOId("<uuid>"))
    ///     // optional: unique id; auto generated by default.
    ///     .with_new_client_order_id("<uuid>")
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_oco_order<'a>(
        &self,
        symbol: &'a str,
        id: ID<'a>,
    ) -> ParamBuilder<'a, '_, CancelOcoParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/orderList").unwrap();

        let order_list_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let list_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                order_list_id,
                list_client_order_id,
                ..Parameters::default()
            },
            client.delete(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get oco order.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use tokio_binance::ID;
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_oco_order(ID::ClientOId("<uuid>"))
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_oco_order<'a>(&self, id: ID<'a>) -> ParamBuilder<'a, '_, OcoStatusParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/orderList").unwrap();

        let order_list_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters {
                order_list_id,
                orig_client_order_id,
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get all oco orders.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::hours(23);
    ///
    /// let response = client
    ///     .get_all_oco_orders()
    ///     // optional: filter by orders greater than or equal to the provided id.
    ///     // If supplied, neither startTime or endTime can be provided
    ///     .with_from_id(1230494)
    ///     // optional: get orders from; pass 24 hours of orders is the default.
    ///     .with_start_time(start)
    ///     // optional: get orders until; default is now.
    ///     .with_end_time(end)
    ///     // optional: limit the amount of orders; default 500; max 1000.
    ///     .with_limit(100)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_all_oco_orders(&self) -> ParamBuilder<'_, '_, AllOcoParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/allOrderList").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get open oco orders.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_open_oco_orders()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_open_oco_orders(&self) -> ParamBuilder<'_, '_, OpenOcoParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/openOrderList").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get current account information.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_account()
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account(&self) -> ParamBuilder<'_, '_, AccountParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/account").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }
    /// Get trades for a specific account and symbol.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{AccountClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::hours(23);
    ///
    /// let response = client
    ///     .get_account_trades("BNBUSDT")
    ///     // optional: filter by orders greater than or equal to the provided id.
    ///     // If supplied, neither startTime or endTime can be provided
    ///     .with_from_id(1230494)
    ///     // optional: get orders from; pass 24 hours of orders is the default.
    ///     .with_start_time(start)
    ///     // optional: get orders until; default is now.
    ///     .with_end_time(end)
    ///     // optional: limit the amount of orders; default 500; max 1000.
    ///     .with_limit(100)
    ///     // optional: processing time for request; default is 5000, can't be above 60000.
    ///     .with_recv_window(8000)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account_trades<'a>(
        &self,
        symbol: &'a str,
    ) -> ParamBuilder<'a, '_, AccountTradesParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/myTrades").unwrap();

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                ..Parameters::default()
            },
            client.get(url),
            Some(api_key),
            Some(secret_key),
        )
    }

    /// Cancel all Open Orders on a Symbol (TRADE)
    /// https://binance-docs.github.io/apidocs/spot/en/#cancel-all-open-orders-on-a-symbol-trade
    /// Fails with 400 when no open orders to cancel.
    pub fn cancel_all_orders<'a>(
        &self,
        symbol: &'a str,
    ) -> ParamBuilder<'a, '_, CancelAllOrdersParams> {
        let Self {
            ref api_key,
            ref secret_key,
            url,
            client,
        } = self;

        let url = url.join("/api/v3/openOrders").unwrap();

        ParamBuilder::new(
            Parameters {
                symbol: Some(symbol),
                ..Parameters::default()
            },
            client.delete(url),
            Some(api_key),
            Some(secret_key),
        )
    }

    /// Helper method for getting a withdraw client instance.
    pub fn to_withdraw_client(&self) -> WithdrawalClient {
        WithdrawalClient {
            api_key: self.api_key.clone(),
            secret_key: self.secret_key.clone(),
            url: self.url.clone(),
            client: self.client.clone(),
        }
    }
    /// Helper method for getting a market client instance.
    pub fn to_market_data_client(&self) -> MarketDataClient {
        MarketDataClient {
            api_key: self.api_key.clone(),
            url: self.url.clone(),
            client: self.client.clone(),
        }
    }
    /// Helper method for getting a general client instance.
    pub fn to_general_client(&self) -> GeneralClient {
        GeneralClient {
            url: self.url.clone(),
            client: self.client.clone(),
        }
    }

    /// Helper method for getting a user data client instance.
    pub fn to_user_data_client(&self) -> UserDataClient {
        UserDataClient {
            api_key: self.api_key.clone(),
            client: self.client.clone(),
            url: self.url.clone(),
        }
    }
}

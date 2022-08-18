use std::hash::{Hash, Hasher};
use finangen_core::prelude::{AccountProxy, format_timestamp, Order, PositionProxy, Transaction};
use finangen_core::prelude::direction::DirectionType;
use finangen_core::prelude::position_effect::PositionEffectType;
use finangen_core::prelude::side::SideType;
use finangen_core::prelude::uuid::Uuid;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AccountRecord{
    pub name: String,
    pub id: Uuid,
    pub available_cash: f64,
    pub frozen_cash: f64,
    pub market_value: f64,
    pub total_value: f64,
    pub transaction_cost: f64,
    pub updated_at: String,
    timestamp: i64,
}

impl From<(&AccountProxy, i64)> for AccountRecord {
    fn from((proxy, updated_at): (&AccountProxy, i64)) -> Self {
        Self{
            name: proxy.name().to_string(),
            id: proxy.id(),
            available_cash: proxy.available_cash(),
            frozen_cash: proxy.frozen_cash(),
            market_value: proxy.market_value(),
            total_value: proxy.total_value(),
            transaction_cost: proxy.transaction_cost(),
            updated_at: format_timestamp(updated_at),
            timestamp: updated_at,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PortfolioRecord{
    pub market_value: f64,
    pub total_value: f64,
    pub transaction_cost: f64,
    pub updated_at: String,
    timestamp: i64,
}

#[derive(Serialize, Clone)]
pub struct PositionRecord{
    pub account_id: Uuid,
    pub code: String,
    pub direction: DirectionType,
    pub avg_price: f64,
    pub quantity: f64,
    pub closable_limited: f64,
    pub closable: f64,
    pub market_value: f64,
    pub transaction_cost: f64,
    pub updated_at: String,
    timestamp: i64,
}

impl From<(Uuid, &PositionProxy, i64)> for PositionRecord {
    fn from((account_id, proxy, updated_at): (Uuid, &PositionProxy, i64)) -> Self {
        Self{
            account_id,
            code: proxy.code().to_string(),
            direction: proxy.direction(),
            avg_price: proxy.avg_price(),
            quantity: proxy.quantity(),
            closable_limited: proxy.closable_limited(),
            closable: proxy.closable(),
            market_value: proxy.market_value(),
            transaction_cost: proxy.transaction_cost(),
            updated_at: format_timestamp(updated_at),
            timestamp: updated_at,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct OrderRecord{
    pub id: Uuid,
    pub account_id: Uuid,
    pub secondary_id: String,
    pub tag: String,
    pub code: String,
    pub side: SideType,
    pub position_effect: PositionEffectType,
    pub direction: DirectionType,

    pub frozen_price: f64,
    pub init_frozen_cash: f64,
    pub avg_price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub transaction_cost: f64,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl Hash for OrderRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl From<&Order> for OrderRecord {
    fn from(o: &Order) -> Self {
        Self{
            id: o.id,
            account_id: o.account.id(),
            secondary_id: o.secondary_id().to_string(),
            tag: o.tag().to_string(),
            code: o.code.clone(),
            side: o.side,
            position_effect: o.position_effect,
            direction: o.direction(),
            frozen_price: o.frozen_price(),
            init_frozen_cash: o.init_frozen_cash(),
            avg_price: o.avg_price(),
            quantity: o.quantity(),
            filled_quantity: o.filled_quantity(),
            transaction_cost: o.transaction_cost(),
            created_at_ts: o.created_at,
            updated_at_ts: o.updated_at(),
            created_at: format_timestamp(o.created_at),
            updated_at: format_timestamp(o.updated_at())
        }
    }
}

impl PartialEq<Self> for OrderRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for OrderRecord {}

#[derive(Serialize, Clone)]
pub struct TransactionRecord{
    pub id: Uuid,
    pub account_id: Uuid,
    pub order_id: Uuid,
    pub secondary_id: String,
    pub code: String,
    pub side: SideType,
    pub position_effect: PositionEffectType,
    pub direction: DirectionType,
    pub price: f64,
    pub frozen_price: f64,
    pub amount: f64,
    pub close_limited_amount: f64,
    pub commission: f64,
    pub tax: f64,
    pub created_at: String,
    pub created_at_ts: i64,
}


impl Hash for TransactionRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl From<&Transaction> for TransactionRecord {
    fn from(o: &Transaction) -> Self {
        Self{
            id: o.id,
            account_id: o.account.id(),
            order_id: o.order.id,
            secondary_id: o.secondary_id.clone(),
            code: o.code.clone(),
            side: o.side,
            position_effect: o.position_effect,
            direction: o.direction(),
            price: o.price,
            frozen_price: o.frozen_price,
            amount: o.amount,
            close_limited_amount: o.close_limited_amount,
            commission: o.commission(),
            tax: o.tax(),
            created_at_ts: o.created_at,
            created_at: format_timestamp(o.created_at),
        }
    }
}

impl PartialEq<Self> for TransactionRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for TransactionRecord {}

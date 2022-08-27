use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use finangen_core::prelude::{Order, Transaction};
use finangen_core::prelude::dashmap::DashSet;
use finangen_core::Runtime;
use crate::models::{AccountRecord, OrderRecord, PositionRecord, TransactionRecord};
use serde::Serialize;

#[derive(Clone)]
pub struct Analyser(Arc<AnalyserInner>);

impl Analyser {
    pub fn new(runtime: &Runtime, benchmark_instruments: HashMap<String, i64>) -> Self{
        Self(Arc::new(AnalyserInner{
            runtime: runtime.clone(),

            benchmark_instruments,
            portfolio: Mutex::new(vec![]),
            benchmark: Mutex::new(vec![]),
            accounts: Mutex::new(Default::default()),
            positions: Mutex::new(Default::default()),
            orders: DashSet::new(),
            transactions: DashSet::new(),
        }))
    }
}
impl Deref for Analyser {
    type Target = AnalyserInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AnalyserInner{
    runtime: Runtime,
    benchmark_instruments: HashMap<String, i64>,
    // 按天统计的投资组合收益率
    portfolio: Mutex<Vec<f64>>,
    // 按天统计的基准收益率
    benchmark: Mutex<Vec<f64>>,
    // 按天统计的账户信息
    accounts: Mutex<HashMap<String, Vec<AccountRecord>>>,
    // 按天统计的持仓信息
    positions: Mutex<HashMap<i64, Vec<PositionRecord>>>,
    // // 按订单触发统计的订单信息
    orders: DashSet<Order>,
    // // 按成交单触发统计的成交单信息
    transactions: DashSet<Transaction>,
}

impl AnalyserInner {

    pub fn collect_daily(&self) {
        // 统计基准净值，用来计算收益率
        {
            let mut benchmark = self.benchmark.lock().unwrap();
            benchmark.push(self.get_benchmark_daily_net_value());
        }
        // 统计组合净值，用来计算收益率，这里需要注意total_value的单位，目前未考虑不同货币间换算的问题
        let accounts = self.runtime.accounts();
        {
            let mut portfolio = self.portfolio.lock().unwrap();
            let mut net_value = 0.0;
            for account in &accounts{
                net_value += account.total_value();
            }
            portfolio.push(net_value);
        }
        // 统计账户信息
        {
            let mut account_records = self.accounts.lock().unwrap();
            let mut positions = self.positions.lock().unwrap();
            let mut position_records = vec![];
            let now = self.runtime.now();
            for account in &accounts{
                if account_records.get_mut(account.name()).is_none(){
                    let _ = account_records.insert(account.name().to_string(),vec![]);
                }
                let account_record = account_records.get_mut(account.name()).unwrap();
                account_record.push(AccountRecord::from((account, now)));
                let account_id = account.id();
                let positions = account.positions(None);
                let mut current_position_records = vec![];
                for position in &positions{
                    let last_price = self.runtime.get_price(position.code()).unwrap().price;
                    current_position_records.push(PositionRecord::from((account_id, position, last_price, now)));
                }
                // position_records.extend(positions.iter().map(|pos| PositionRecord::from((account_id, pos, now))).collect::<Vec<PositionRecord>>());
                position_records.extend(current_position_records);
            }
            positions.insert(self.runtime.now(), position_records);
        }
    }

    pub fn collect_order(&self, order: &Order) {
        let _ = self.orders.insert(order.clone());
    }

    pub fn collect_transaction(&self, transaction: &Transaction){
        let _ = self.transactions.insert(transaction.clone());
    }
}

impl AnalyserInner {
    // 获取到的应当是当前份额
    fn get_benchmark_daily_net_value(&self) -> f64{
        let mut weights = 0;
        let mut benchmark = 0.0;
        for (code, weight) in &self.benchmark_instruments{
            if let Some(price_record) = self.runtime.get_price(code){
                benchmark += price_record.price * *weight as f64;
            }
            weights += weight;
        }
        benchmark / weights as f64
    }
}

#[derive(Serialize)]
pub struct Snapshot{
    benchmark_instruments: HashMap<String, i64>,
    portfolio_net_value: Vec<f64>,
    benchmark_net_value: Vec<f64>,
    accounts: HashMap<String, Vec<AccountRecord>>,
    positions: HashMap<i64, Vec<PositionRecord>>,
    orders: Vec<OrderRecord>,
    transactions: Vec<TransactionRecord>,
}

impl AnalyserInner {
    pub fn get_snapshot(&self) -> Snapshot{
        let portfolio_net_value = {
            self.portfolio.lock().unwrap().clone()
        };
        let benchmark_net_value = {
            self.benchmark.lock().unwrap().clone()
        };
        let accounts = {
            self.accounts.lock().unwrap().clone()
        };
        let positions = {
            self.positions.lock().unwrap().clone()
        };
        let orders = {
            self.orders.iter().map(|v|OrderRecord::from(v.key())).collect::<Vec<OrderRecord>>()
        };
        let transactions = {
            self.transactions.iter().map(|v| TransactionRecord::from(v.key())).collect::<Vec<TransactionRecord>>()
        };
        Snapshot{
            benchmark_instruments: self.benchmark_instruments.clone(),
            portfolio_net_value,
            benchmark_net_value,
            accounts,
            positions,
            orders,
            transactions,
        }
    }
}

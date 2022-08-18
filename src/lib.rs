mod configs;
mod models;
mod analysers;

use std::sync::{Arc, Mutex};
use finangen_core::payloads::common::{OrderPayload, TransactionPayload};
use finangen_core::prelude::{Cron, Event, event_types, SubscriptionMode};
use finangen_core::Runtime;
use finangen_plugins::{declare_plugin, Plugin};
use finangen_plugins::prelude::Serialize;
use crate::analysers::Analyser;
use crate::configs::Config;

declare_plugin!(AnalysisPlugin, AnalysisPlugin::new);


pub struct AnalysisPlugin {
    analyser: Mutex<Option<Analyser>>,
}

impl AnalysisPlugin {
    pub fn new() -> Self{
        Self{
            analyser: Mutex::new(None)
        }
    }
}

impl Plugin for AnalysisPlugin {
    fn name(&'_ self) -> &'_ str {
        "finangen_analysis_plugin"
    }

    fn install(&self, runtime: &Runtime) {
        let config = runtime.
            config.
            get::<Config>("plugins.finangen_analysis_plugin").
            expect("cannot find plugin config key: plugins.finangen_analysis_plugin");
        let cron_expr = match config.daily_collect_cron_expr {
            None => {
                "0 0 0 * * *".to_string()
            }
            Some(expr) => expr
        };
        let analyser = Analyser::new(runtime, config.benchmark);
        {
            let analyser_ref = analyser.clone();
            runtime.subscribe(event_types::TRADE, move |e: Event|{
                let payload = e.payload::<TransactionPayload>();
                analyser_ref.collect_transaction(&payload.transaction);
            }, None, SubscriptionMode::System);
        }
        {
            let analyser_ref = analyser.clone();
            runtime.subscribe(event_types::ORDER_CREATION_PASS, move |e: Event|{
                let payload = e.payload::<OrderPayload>();
                analyser_ref.collect_order(&payload.order);
            }, None, SubscriptionMode::System);
        }
        // {
        //     let analyser_ref = analyser.clone();
        //     runtime.subscribe(event_types::POST_USER_INIT, move |_e: Event| {
        //         analyser_ref.collect_daily();
        //     }, None, SubscriptionMode::System)
        // }
        {
            let analyser_ref = analyser.clone();
            runtime.schedule_block(move || {
                analyser_ref.collect_daily();
            }, Cron::local(runtime.now(),&cron_expr, None));
        }
        let mut global_analyser = self.analyser.lock().unwrap();
        *global_analyser = Some(analyser);
    }

    fn uninstall(&self) -> Option<Arc<dyn Serialize>> {
        let analyser = {
            let analyser = self.analyser.lock().unwrap();
            analyser.as_ref().unwrap().clone()
        };
        let snapshot = analyser.get_snapshot();
        Some(Arc::new(snapshot))
    }
}

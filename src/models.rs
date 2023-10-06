use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolRecord {
	ask: f32,
	bid: f32,
	category_name: String,
	contract_size: i32,
	currency: String,
	currency_pair: bool,
	currency_profit: String,
	description: String,
	expiration: Option<String>,
	group_name: String,
	high: f32,
	initial_margin: i32,
	instant_max_volume: i32,
	leverage: f32,
	long_only: bool,
	lot_max: f32,
	lot_min: f32,
	lot_step: f32,
	low: f32,
	margin_hedged: i32,
	margin_hedged_strong: bool,
	margin_maintenance: Option<i32>,
	margin_mode: i32,
	percentage: f32,
	precision: i32,
	profit_mode: i32,
	quote_id: i32,
	short_selling: bool,
	spread_raw: f32,
	spread_table: f32,
	starting: Option<u64>,
	step_rule_id: i32,
	stops_level: i32,
    #[serde(rename = "swap_rollover3days")]
	swap_rollover3days: i32,
	swap_enable: bool,
	swap_long: f32,
	swap_short: f32,
	swap_type: i32,
	symbol: String,
	tick_size: f32,
	tick_value: f32,
	time: u64,
	time_string: String,
	trailing_enabled: bool,
    #[serde(rename = "type")]
    instrument_class: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolResult {
	status: bool,
	pub return_data: Vec<SymbolRecord>	
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeTransInfo {
	cmd: i32,
	custom_comment: String,
	expiration: u64,
	offset: i32,
	order: i32,
	price: f32,
	sl: f32,
	symbol: String,
	tp: f32,
	#[serde(rename = "type")]
	transaction_type: i32,
	volume: f32
}

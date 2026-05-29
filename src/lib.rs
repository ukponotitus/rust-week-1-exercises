use std::collections::HashMap;

// Name Assignment (variables and constants)
// TODO: Assign the current bitcoin mining reward
// As of May 2026 (post-2024 halving), the reward is 3.125 BTC
pub const MINING_REWARD: f64 = 3.125;
// TODO: Assign the current block height
// Approximate block height for May 2026 (840,000 + ~2.1 years of blocks)
pub const CURRENT_BLOCK_HEIGHT: u64 = 951_239;
// TODO: Assign the number of satoshis in one Bitcoin
// 1 Bitcoin = 100,000,000 Satoshis
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    // TODO: Multiply blocks_mined by MINING_REWARD and return result
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    (0.00001..=0.01).contains(&fee)
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    // TODO: Compare balance to 50.0 and return result
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    // TODO: Calculate fee rate (fee_btc / size_bytes) and use if/else if/else
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    let fee_rate = fee_btc / size_bytes as f64;
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    // TODO: Convert network to lowercase and compare with "mainnet"
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    // TODO: Check if 100 <= value <= 200
    (100..=200).contains(&value)
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    // TODO: Use std::ptr::eq to compare reference identity
    std::ptr::eq(wallet1, wallet2)
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    // TODO: Trim leading/trailing whitespace and convert to lowercase
    address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    // TODO: Push new_utxo into utxos and return it
    utxos.push(new_utxo);
    utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    // TODO: Iterate with enumerate and return the first (index, fee) where fee > 0.005
    fee_list
        .iter()
        .enumerate()
        .find(|(_, &fee)| fee > 0.005)
        .map(|(i, &f)| (i, f))
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    // TODO: Return a tuple with wallet name and balance
    ("satoshi_wallet".to_string(), 50.0)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    // TODO: Look up txid in tx_pool, returning the status or "not found"
    tx_pool
        .get(txid)
        .cloned()
        .unwrap_or_else(|| "not found".to_string())
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // TODO: Destructure the tuple into (name, balance) and format the result
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    // TODO: Multiply btc by BTC_TO_SATS and return as u64
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    // TODO: Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    // TODO: Concatenate prefix + suffix and return
    let charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let suffix: String = (0..(32 - prefix.len()))
        .map(|i| charset.chars().nth(i % charset.len()).unwrap())
        .collect();
    format!("{}{}", prefix, suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    // TODO: Check that height is not negative
    // TODO: Check that height is within a realistic range (<= 800_000)
    // TODO: Return (true, "Valid block height") otherwise
    if height < 0 {
        (false, "Negative height".to_string())
    } else if height > 800_000 {
        (false, "Unrealistic block height".to_string())
    } else {
        (true, "Valid block height".to_string())
    }
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    // TODO: Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    // TODO: For each block: halvings = block / 210_000; reward = base >> halvings
    // TODO: Insert (block, reward) into the result HashMap
    let mut map = HashMap::new();
    let base_reward: u64 = 50 * BTC_TO_SATS;

    for &block in blocks {
        let halvings = block / 210_000;
        // Bit shift to halve (reward >> halvings). Handle overflow for large halvings.
        let reward = if halvings >= 64 {
            0
        } else {
            base_reward >> halvings
        };
        map.insert(block, reward);
    }
    map
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    // TODO: Filter UTXOs to those with value >= target
    // TODO: Return the one with the smallest value, or None if none qualify
    utxos
        .iter()
        .filter(|u| u.value >= target)
        .min_by_key(|u| u.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // TODO: Build a base map with "txid" and "vout" (as string)
    // TODO: Merge extra into the base map and return
    let mut map = HashMap::new();
    map.insert("txid".to_string(), txid.to_string());
    map.insert("vout".to_string(), vout.to_string());
    map.extend(extra);
    map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Bitcoin uses Little Endian for versions.
    // Example: "01000000" is Version 1.
    let mut bytes = [0u8; 4];
    for i in 0..4 {
        bytes[i] = u8::from_str_radix(&raw_tx_hex[i * 2..i * 2 + 2], 16)
            .map_err(|_| "Hex decode error".to_string())?;
    }

    Ok(u32::from_le_bytes(bytes))
}

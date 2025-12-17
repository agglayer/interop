use agglayer_primitives::{Address, Digest, U256};
use agglayer_tries::{error::SmtError, smt::Smt};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{NetworkId, TokenInfo};

pub struct BalanceTree(pub Smt<192>);

impl BalanceTree {
    /// Returns all the non-zero token balance contained in this balance tree.
    pub fn get_all_balances(&self) -> Result<Vec<TokenBalanceEntry>, SmtError> {
        Ok(self
            .0
            .entries()?
            .into_iter()
            .map(TokenBalanceEntry::from)
            .collect())
    }

    /// Returns the balance for the given [`TokenInfo`].
    pub fn get_balance(&self, token_info: TokenInfo) -> TokenBalanceEntry {
        let amount = self
            .0
            .get(token_info)
            .map(|v| U256::from_be_bytes(*v.as_bytes()))
            .unwrap_or(U256::ZERO);

        TokenBalanceEntry {
            origin_network: token_info.origin_network,
            origin_token_address: token_info.origin_token_address,
            amount,
        }
    }
}

/// Token balance entry structure in order to display the balance tree values.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceEntry {
    pub origin_network: NetworkId,
    pub origin_token_address: Address,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: U256,
}

impl From<([bool; 192], Digest)> for TokenBalanceEntry {
    fn from((path, leaf_value): ([bool; 192], Digest)) -> Self {
        let TokenInfo {
            origin_network,
            origin_token_address,
        } = TokenInfo::from_bits(&path);

        Self {
            origin_network,
            origin_token_address,
            amount: U256::from_be_bytes(*leaf_value.as_bytes()),
        }
    }
}

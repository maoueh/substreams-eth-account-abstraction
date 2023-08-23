mod abi;
mod pb;
use abi::contract::functions;
use hex_literal::hex;
use pb::contract::v1::{self as contract, UserOperation, UserOperations};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Function;

const TRACKED_CONTRACT: [u8; 20] = hex!("5ff137d4b0fdcd49dca30c7cf57e578a026d2789");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::UserOperations, substreams::errors::Error> {
    Ok(UserOperations {
        operations: extract_user_operations(&blk),
    })
}

fn extract_user_operations(blk: &eth::Block) -> Vec<UserOperation> {
    blk.transaction_traces
        .iter()
        .flat_map(|trx| {
            let trx_hash = Hex::encode(&trx.hash);

            trx.calls.iter().flat_map(move |call| {
                if call.state_reverted || call.address != TRACKED_CONTRACT {
                    return vec![];
                }

                if let Some(args) = functions::HandleOps::match_and_decode(call) {
                    return args
                        .ops
                        .into_iter()
                        .map(|op| (trx_hash.clone(), call.index, op))
                        .map(Into::into)
                        .collect();
                }

                vec![]
            })
        })
        .collect()
}

impl
    From<(
        String,
        u32,
        (
            Vec<u8>,
            substreams::scalar::BigInt,
            Vec<u8>,
            Vec<u8>,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            Vec<u8>,
            Vec<u8>,
        ),
    )> for UserOperation
{
    fn from(
        tuple: (
            String,
            u32,
            (
                Vec<u8>,
                substreams::scalar::BigInt,
                Vec<u8>,
                Vec<u8>,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<u8>,
                Vec<u8>,
            ),
        ),
    ) -> Self {
        let op = tuple.2;

        Self {
            trx_hash: tuple.0,
            call_index: tuple.1 as u64,

            sender: Hex::encode(op.0),
            nonce: op.1.to_string(),
            init_code: Hex::encode(op.2),
            call_data: Hex::encode(op.3),
            call_gas_limit: op.4.to_string(),
            verification_gas_limit: op.5.to_string(),
            pre_verification_gas: op.6.to_string(),
            max_fee_per_gas: op.7.to_string(),
            max_priority_fee_per_gas: op.8.to_string(),
            paymaster_and_data: Hex::encode(op.9),
            signature: Hex::encode(op.10),
        }
    }
}

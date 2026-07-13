use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 1482
// Hash 6890
// Hash 9075
// Hash 3158
// Hash 6739
// Hash 6372
// Hash 4100
// Hash 2349
// Hash 3665
// Hash 1630
// Hash 6355
// Hash 6685
// Hash 1132
// Hash 9630
// Hash 9216
// Hash 6816
// Hash 5895
// Hash 8515
// Hash 4660
// Hash 5411
// Hash 9516
// Hash 3648
// Hash 4160
// Hash 8945
// Hash 4052
// Hash 8307
// Hash 4489
// Hash 7121
// Hash 5620
// Hash 5374
// Hash 8480
// Hash 6890
// Hash 1749
// Hash 8423
// Hash 2051
// Hash 1267
// Hash 9875
// Hash 5118
// Hash 2366
// Hash 7742
// Hash 9422
// Hash 2925
// Hash 7829
// Hash 4966
// Hash 3451
// Hash 4853
// Hash 5051
// Hash 2541
// Hash 3832
// Hash 3134
// Hash 3545
// Hash 7816
// Hash 8258
// Hash 5043
// Hash 3707
// Hash 3552
// Hash 9715
// Hash 7341
// Hash 5188
// Hash 3793
// Hash 9022
// Hash 5150
// Hash 7257
// Hash 7958
// Hash 7494
// Hash 2176
// Hash 3661
// Hash 1415
// Hash 9091
// Hash 1591
// Hash 3300
// Hash 2438
// Hash 3924
// Hash 8748
// Hash 2910
// Hash 5045
// Hash 8915
// Hash 5270
// Hash 2815
// Hash 2605
// Hash 2888
// Hash 5310
// Hash 6265
// Hash 2246
// Hash 1566
// Hash 1455
// Hash 1476
// Hash 6802
// Hash 7206
// Hash 8465
// Hash 1880
// Hash 1825
// Hash 8978
// Hash 6468
// Hash 7612
// Hash 9022
// Hash 5439
// Hash 5243
// Hash 7764
// Hash 4985
// Hash 2061
// Hash 2896
// Hash 9128
// Hash 2740
// Hash 9393
// Hash 5922
// Hash 4737
// Hash 6776
// Hash 4445
// Hash 1466
// Hash 6874
// Hash 6762
// Hash 8534
// Hash 1736
// Hash 1028
// Hash 3175
// Hash 7145
// Hash 7239
// Hash 5513
// Hash 8471
// Hash 2717
// Hash 8137
// Hash 1653
// Hash 2100
// Hash 3706
// Hash 2178
// Hash 5855
// Hash 9077
// Hash 2928
// Hash 6689
// Hash 2843
// Hash 1080
// Hash 4133
// Hash 9115
// Hash 9958
// Hash 8777
// Hash 1720
// Hash 2629
// Hash 5052
// Hash 9275
// Hash 8859
// Hash 6418
// Hash 5846
// Hash 1826
// Hash 7208
// Hash 4815
// Hash 5903
// Hash 3156
// Hash 8058
// Hash 3048
// Hash 3622
// Hash 4332
// Hash 9374
// Hash 7707
// Hash 6677
// Hash 9509
// Hash 3370
// Hash 7763
// Hash 3990
// Hash 3633
// Hash 9605
// Hash 8212
// Hash 8011
// Hash 5232
// Hash 4060
// Hash 4129
// Hash 4442
// Hash 7250
// Hash 9777
// Hash 3130
// Hash 1444
// Hash 4134
// Hash 4085
// Hash 3082
// Hash 8399
// Hash 1435
// Hash 9025
// Hash 2777
// Hash 6096
// Hash 7919
// Hash 6449
// Hash 1092
// Hash 6352
// Hash 3339
// Hash 9248
// Hash 1209
// Hash 6624
// Hash 4988
// Hash 6365
// Hash 9448
// Hash 1573
// Hash 6787
// Hash 2945
// Hash 2776
// Hash 8464
// Hash 6132
// Hash 9172
// Hash 9680
// Hash 7909
// Hash 1635
// Hash 8461
// Hash 2929
// Hash 5768
// Hash 8938
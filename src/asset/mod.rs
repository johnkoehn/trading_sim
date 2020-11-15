use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Asset {
    BTC,
    ETH,
    USD
}

impl Asset {
    pub fn iterator() -> Iter<'static, Asset> {
        static ASSETS: [Asset; 3] = [Asset::BTC, Asset::ETH, Asset::USD];
        ASSETS.iter()
    }
}
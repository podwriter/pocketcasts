/// User subscription status.
#[derive(Clone, Debug, Deserialize)]
pub struct SubscriptionStatus {
    pub paid: i64,
    pub platform: i64,
    #[serde(rename = "expiryDate")]
    pub expiry_date: String,
    #[serde(rename = "autoRenewing")]
    pub auto_renewing: bool,
    #[serde(rename = "giftDays")]
    pub gift_days: i64,
    #[serde(rename = "cancelUrl")]
    pub cancel_url: String,
    #[serde(rename = "updateUrl")]
    pub update_url: String,
    pub frequency: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StrategyType {
    LowestLatency,
    HighestFidelity,
    HighestSuccess,
}
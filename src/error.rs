// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfPlotterError {
    pub msg: String,
}

impl std::fmt::Display for PerfPlotterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl std::error::Error for PerfPlotterError {}

impl PerfPlotterError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl From<&str> for PerfPlotterError {
    fn from(msg: &str) -> Self {
        Self::new(msg.to_string())
    }
}

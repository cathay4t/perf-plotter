// SPDX-License-Identifier: Apache-2.0

// use plotters::prelude::*;

mod error;
mod perf;

pub use self::error::PerfPlotterError;
pub use self::perf::generate_performance_png;

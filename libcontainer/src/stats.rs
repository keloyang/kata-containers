// Copyright (c) 2019 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

// use crate::cgroups::Stats as CgroupStats;
// use crate::intelrdt::Stats as RdtStats;

struct CgroupStats {
}

struct RdtStats {
}

pub struct NetworkInterface {
	name: String,
	rx_bytes: u64,
	rx_packets: u64,
	rx_errors: u64,
	rx_dropped: u64,
	tx_bytes: u64,
	tx_packets: u64,
	tx_errors: u64,
	tx_dropped: u64,
}

pub struct Stats {
	interfaces: Vec<NetworkInterface>,
	cgroup_stats: Option<CgroupStats>,
	intel_rdt_stats: Option<RdtStats>,
}

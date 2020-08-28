// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
//! This feature gets turned on only if consensus-types is compiled via MIRAI in a nightly build.
#![cfg_attr(mirai, allow(incomplete_features), feature(const_generics))]

pub mod block;
pub mod block_data;
pub mod block_retrieval;
pub mod common;
pub mod epoch_retrieval;
pub mod executed_block;
pub mod proposal_msg;
pub mod quorum_cert;
pub mod safety_data;
pub mod sync_info;
pub mod timeout;
pub mod timeout_certificate;
pub mod vote;
pub mod vote_data;
pub mod vote_msg;
pub mod vote_proposal;

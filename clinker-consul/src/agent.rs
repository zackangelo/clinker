#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde(default)]
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentCheck {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
}

#[serde(default)]
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentMember {
    pub Name: String,
    pub Addr: String,
    pub Port: u16,
    pub Tags: HashMap<String, String>,
    pub pubStatus: usize,
    pub ProtocolMin: u8,
    pub ProtocolMax: u8,
    pub ProtocolCur: u8,
    pub DelegateMin: u8,
    pub DelegateMax: u8,
    pub DelegateCur: u8,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentService {
    pub ID: String,
    pub Service: String,
    pub Tags: Option<Vec<String>>,
    pub Port: u16,
    pub Address: String,
    pub EnableTagOverride: bool,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

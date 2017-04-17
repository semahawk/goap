//
// lib.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Apr 2017 12:45:39 +0200 (CEST)
//

use std::collections::HashMap;

use std::hash::Hash;
use std::cmp::{Eq, PartialEq};

#[derive(Debug)]
pub struct ActionPlanner<A, C>
where A: Hash + Eq + PartialEq {
  actions: HashMap<A, (Vec<C>, Vec<C>)>,
}

impl<A, C> ActionPlanner<A, C>
where A: Hash + Eq + PartialEq {
  pub fn new() -> ActionPlanner<A, C> {
    ActionPlanner {
      actions: HashMap::new(),
    }
  }

  pub fn add_action(&mut self, action: A, preconditions: Vec<C>, effects: Vec<C>) {
    self.actions.insert(action, (preconditions, effects));
  }

  pub fn display_actions(&self)
  where A: std::fmt::Debug, C: std::fmt::Debug {
    for (ref action, ref cond) in &self.actions {
      println!("action: {:?}", action);
      println!("-- preconditions: {:?}", cond.0);
      println!("-- effects: {:?}", cond.1);
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */


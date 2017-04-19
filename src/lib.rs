//
// lib.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Apr 2017 12:45:39 +0200 (CEST)
//

use std::collections::{HashMap, HashSet, BinaryHeap, BTreeSet};

use std::hash::Hash;
use std::cmp::{Ordering, Eq, PartialEq};

#[derive(Debug)]
pub struct ActionPlanner<A, C>
where A: Hash + Eq + PartialEq,
      C: Hash + Eq + PartialEq + Clone {
  actions: HashMap<A, (BTreeSet<C>, BTreeSet<C>, usize)>,
  states: BTreeSet<C>,
  goal: BTreeSet<C>,
}

impl<A, C> ActionPlanner<A, C>
where A: Hash + Eq + PartialEq,
      C: Hash + Eq + PartialEq + Clone + Ord {
  pub fn new() -> ActionPlanner<A, C> {
    ActionPlanner {
      actions: HashMap::new(),
      states: BTreeSet::new(),
      goal: BTreeSet::new(),
    }
  }

  pub fn add_action(&mut self, action: A, preconditions: Vec<C>, effects: Vec<C>, cost: usize) {
    let mut new_preconditions = BTreeSet::new();
    let mut new_effects = BTreeSet::new();

    for precondition in preconditions {
      new_preconditions.insert(precondition);
    }

    for effect in effects {
      new_effects.insert(effect);
    }

    self.actions.insert(action, (new_preconditions, new_effects, cost));
  }

  pub fn set_state(&mut self, state: C) {
    self.states.insert(state);
  }

  pub fn set_goal(&mut self, goal: C) {
    self.goal.insert(goal);
  }


  pub fn plan_next_step(&mut self) -> Vec<&A>
  where A: std::fmt::Debug, C: std::fmt::Debug {
    println!("===");
    println!("=== finding a way to {:?}", self.goal);
    println!("===");
    let mut plan: Vec<&A> = Vec::new();

    fn approx_distance_to<C>(from: &BTreeSet<C>, goal: &BTreeSet<C>) -> usize
    where C: std::cmp::Ord + std::fmt::Debug {
      goal.difference(from).count()
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Node<T>
    where T: Hash + Eq + PartialEq {
      states: BTreeSet<T>,
      cost: usize,
    }

    impl<T> Ord for Node<T>
    where T: Hash + Eq {
      fn cmp(&self, other: &Node<T>) -> Ordering {
        other.cost.cmp(&self.cost)
      }
    }

    impl<T> PartialOrd for Node<T>
    where T: Hash + Eq + PartialEq {
      fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }

    let mut came_from: HashMap<BTreeSet<C>, Option<Node<C>>> = HashMap::new();
    let mut cost_so_far: HashMap<BTreeSet<C>, usize> = HashMap::new();

    let mut frontier = BinaryHeap::new();

    came_from.insert(self.states.clone(), None);
    cost_so_far.insert(self.states.clone(), 0);
    frontier.push(Node { states: self.states.clone(), cost: 0 });

    'find_plan: while let Some(current_node) = frontier.pop() {
      println!("-- popped node from the frontier: {:?}", current_node);

      'find_action: for (action, &(ref preconds, ref effects, cost)) in &self.actions {
        println!("---- checking action {:?} ({:?} -> {:?})", action, preconds, effects);
        for precond in preconds {
          if let None = self.states.get(precond) {
            continue 'find_action;
          }
        }

        println!("------ fulfills all preconditions!");

        //println!("current_node.states vs the goal: {:?} vs {:?}", current_node.states, self.goal);

        let new_cost = cost_so_far.get(&current_node.states).unwrap() + cost;
        if !cost_so_far.contains_key(&preconds) || new_cost < *cost_so_far.get(&preconds).unwrap() {
          //println!("cost_so_far: {:?}", cost_so_far);
          //println!("quote next: {:?}", preconds);
          let mut new_state = self.states.clone();
          for effect in effects {
            new_state.insert(effect.clone());
          }
          cost_so_far.insert(preconds.clone(), new_cost);
          println!("------ pushing {:?} onto the frontier", action);
          frontier.push(Node { states: new_state, cost: new_cost + approx_distance_to(effects, &self.goal) });
        }

        plan.push(action);
      }
    }

    plan
  }

  pub fn display_actions(&self)
  where A: std::fmt::Debug, C: std::fmt::Debug {
    for (ref action, &(ref preconds, ref effects, ..)) in &self.actions {
      println!("action: {:?}", action);
      println!("-- preconditions: {:?}", preconds);
      println!("-- effects: {:?}", effects);
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */


//
// lib.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Apr 2017 12:45:39 +0200 (CEST)
//

use std::collections::{HashMap, HashSet, BinaryHeap, BTreeMap};

use std::hash::Hash;
use std::cmp::{Ordering, Eq, PartialEq};

#[derive(Debug)]
pub struct ActionPlanner<A, C>
where A: Hash + Eq + PartialEq + Clone,
      C: Hash + Eq + PartialEq + Clone {
  actions: HashMap<A, (BTreeMap<C, bool>, BTreeMap<C, bool>, usize)>,
  states: BTreeMap<C, bool>,
  goal: BTreeMap<C, bool>,
}

impl<A, C> ActionPlanner<A, C>
where A: Hash + Eq + PartialEq + Clone,
      C: Hash + Eq + PartialEq + Clone + Ord {
  pub fn new() -> ActionPlanner<A, C> {
    ActionPlanner {
      actions: HashMap::new(),
      states: BTreeMap::new(),
      goal: BTreeMap::new(),
    }
  }

  pub fn add_action(&mut self, action: A, preconditions: Vec<(C, bool)>, effects: Vec<(C, bool)>, cost: usize) {
    let mut new_preconditions = BTreeMap::new();
    let mut new_effects = BTreeMap::new();

    for (precondition, value) in preconditions {
      new_preconditions.insert(precondition, value);
    }

    for (effect, value) in effects {
      new_effects.insert(effect, value);
    }

    self.actions.insert(action, (new_preconditions, new_effects, cost));
  }

  pub fn set_state(&mut self, state: (C, bool)) {
    self.states.insert(state.0, state.1);
  }

  pub fn set_goal(&mut self, goal: (C, bool)) {
    self.goal.insert(goal.0, goal.1);
  }


  pub fn plan(&mut self) -> Vec<A>
  where A: std::fmt::Debug, C: std::fmt::Debug {
    println!("===");
    println!("=== finding a way to {:?}", self.goal);
    println!("===");
    let mut plan: Vec<A> = Vec::new();

    fn approx_distance_to<C>(from: &BTreeMap<C, bool>, goal: &BTreeMap<C, bool>) -> usize
    where C: std::cmp::Ord + std::fmt::Debug {
      //goal.difference(from).count()
      10
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Node<U, T>
    where T: Hash + Eq + PartialEq,
          U: Hash + Eq + PartialEq + Clone {
      action: Option<U>,
      states: BTreeMap<T, bool>,
      cost: usize,
      parent: Option<Box<Node<U, T>>>,
    }

    impl<U, T> Ord for Node<U, T>
    where T: Hash + Eq,
          U: Hash + Eq + Clone {
      fn cmp(&self, other: &Node<U, T>) -> Ordering {
        other.cost.cmp(&self.cost)
      }
    }

    impl<U, T> PartialOrd for Node<U, T>
    where T: Hash + Eq + PartialEq,
          U: Hash + Eq + PartialEq + Clone {
      fn partial_cmp(&self, other: &Node<U, T>) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }

    let mut last_step: Option<Box<Node<A, C>>> = None;
    //let mut came_from: HashMap<BTreeMap<C, bool>, Option<Node<A, C>>> = HashMap::new();
    let mut cost_so_far: HashMap<BTreeMap<C, bool>, usize> = HashMap::new();

    let mut frontier = BinaryHeap::new();

    //came_from.insert(self.states.clone(), None);
    cost_so_far.insert(self.states.clone(), 0);
    frontier.push(Node { action: None, states: self.states.clone(), cost: 0, parent: None });

    'find_plan: while let Some(current_node) = frontier.pop() {
      //println!("-- popped node from the frontier: {:?}", current_node);

      'find_action: for (action, &(ref preconds, ref effects, cost)) in &self.actions {
        //println!("current_node.states: {:?}", current_node.states);
        //println!("---- checking action {:?} ({:?} -> {:?})", action, preconds, effects);
        for precond in preconds {
          match current_node.states.get(precond.0) {
            Some(state) => {
              if state != precond.1 { continue 'find_action; }
            },
            None => continue 'find_action,
          }
        }

        //println!("------ {:?} ({:?} -> {:?}) fulfills all preconditions!", action, preconds, effects);

        //println!("current_node.states vs the goal: {:?} vs {:?}", current_node.states, self.goal);
        {
          let mut goal_found = true;
          for (state, value) in &self.goal {
            if let Some(a) = current_node.states.get(state) {
              if a != value {
                goal_found = false;
                break;
              }
            }
            if let None = current_node.states.get(state) {
              goal_found = false;
              break;
            }
          }

          if goal_found {
            println!("FOUND THE GOAL!");
            last_step = Some(Box::new(current_node));
            break 'find_plan;
          }
        }

        let new_cost = cost_so_far.get(&current_node.states).unwrap() + cost;
        if !cost_so_far.contains_key(&preconds) || new_cost < *cost_so_far.get(&preconds).unwrap() {
          let mut new_state = current_node.states.clone();
          for (effect, value) in effects {
            new_state.insert(effect.clone(), *value);
          }
          cost_so_far.insert(new_state.clone(), new_cost);
          let new_node = Node { action: Some(action.clone()), states: new_state, cost: new_cost + approx_distance_to(effects, &self.goal), parent: Some(Box::new(current_node.clone())) };
          frontier.push(new_node);
        }
      }
    }

    let mut node = last_step.unwrap();
    loop {
      let action = node.clone().action;

      if !action.is_none() {
        plan.push(action.unwrap().clone());
      }

      if node.parent.is_none() { break }
      node = node.parent.unwrap();
    }

    plan.reverse();
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


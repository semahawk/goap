//
// simple.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Apr 2017 13:12:48 +0200 (CEST)
//

extern crate goap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Action {
  Attack,
  Flee,
  Explode,
  FindWeapon,
  Approach,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
enum Condition {
  HasWeapon,
  NearEnemy,
  EnemyAlive,
}

fn main() {
  let mut ap = goap::ActionPlanner::new();

  ap.add_action(
    Action::Explode,
    vec!((Condition::NearEnemy, true)),
    vec!((Condition::EnemyAlive, false)),
    11
  );

  ap.add_action(
    Action::Attack,
    vec!((Condition::HasWeapon, true), (Condition::NearEnemy, true)),
    vec!((Condition::EnemyAlive, false)),
    10
  );

  ap.add_action(
    Action::Flee,
    vec!((Condition::HasWeapon, false), (Condition::NearEnemy, true)),
    vec!((Condition::NearEnemy, false)),
    10
  );

  ap.add_action(
    Action::FindWeapon,
    vec!((Condition::HasWeapon, false), (Condition::NearEnemy, false)),
    vec!((Condition::HasWeapon, true)),
    10
  );

  ap.add_action(
    Action::Approach,
    vec!((Condition::HasWeapon, true)),
    vec!((Condition::NearEnemy, true)),
    10
  );

  ap.set_state((Condition::NearEnemy, true));
  ap.set_state((Condition::HasWeapon, true));
  ap.set_state((Condition::EnemyAlive, true));

  ap.set_goal((Condition::EnemyAlive, false));

  ap.display_actions();

  println!("action plan: {:?}", ap.plan());
}

/*
 * vi: ts=2 sw=2 expandtab
 */


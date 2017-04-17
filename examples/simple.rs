//
// simple.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Apr 2017 13:12:48 +0200 (CEST)
//

extern crate goap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Action {
  Attack,
  Flee,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Condition {
  HasWeapon(bool),
  NearEnemy(bool),
  EnemyAlive(bool),
}

fn main() {
  let mut ap = goap::ActionPlanner::new();

  ap.add_action(
    Action::Attack,
    vec!(Condition::HasWeapon(true), Condition::NearEnemy(true)),
    vec!(Condition::EnemyAlive(false))
  );

  ap.add_action(
    Action::Flee,
    vec!(Condition::HasWeapon(false)),
    vec!(Condition::NearEnemy(false))
  );

  ap.set_state(Condition::NearEnemy(true));
  ap.set_state(Condition::HasWeapon(true));

  ap.set_goal(Condition::EnemyAlive(false));

  ap.display_actions();
}

/*
 * vi: ts=2 sw=2 expandtab
 */


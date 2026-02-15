#![feature(test)]
extern crate test;

use test::Bencher;

use shobu::logic::game::GameState;
use shobu::logic::movegen::MoveGen;
use shobu::logic::control::agent::*;
use shobu::logic::control::evaluation::*;

#[bench]
fn bench_minimax(b: &mut Bencher) {
	let mut movegen = MoveGen::new();
	let mut agent = MinimaxAgent::new(2, simple);
	b.iter(move || agent.act(&mut movegen, GameState::initial()));
}

#![feature(trait_alias)]
pub mod select;

use yew::prelude::*;
use select::Select;

use shobu::logic;
use shobu::logic::game::{Color, GameState};
use shobu::logic::movegen::{Move, MoveGen};
use shobu::logic::board::{Board, Square};

use shobu::logic::control::{Players, Player};


enum Msg {
	Select(Square),
	Deselect,

	Surrender,

	Undo,
	Next,
	Reset,
}

struct App {
	state: GameState,
	history: Vec<GameState>,

	players: Players,

	surrendered: Option<Color>,

	gen: MoveGen,
	moves: Vec<Move>,

	possible: Vec<Square>,
	selected_passive: Option<Square>,
	selected_active: Option<Square>,

	last_move: Option<Move>,
}

impl App {
	pub fn reset(&mut self) {
		self.state = GameState::initial();
		self.history = vec![];
		self.last_move = None;
		self.surrendered = None;
		self.computer_turn();
	}

	pub fn undo(&mut self) -> bool {
		match self.history.pop() {
			Some(state) => {
				self.state = state;
				self.last_move = None;
				self.reset_moves();
				true
			},
			None => false,
		}
	}

	pub fn computer_turn(&mut self) {
		self.state = match self.players.get(self.state.current_side) {
			Player::Human => self.state,
			Player::Computer(ref mut strat) => match strat.act(&mut self.gen, self.state) {
				None => self.state,
				Some(mv) => {
					self.last_move = Some(mv);
					mv.apply(self.state).expect("bad state: invalid move")
				},
			}
		};
		self.reset_moves();
	}

	pub fn human_turn(&mut self, mv: Move) -> bool {
		self.last_move = Some(mv);

		match mv.apply(self.state) {
			Some(state) => {
				self.history.push(self.state);

				self.state = state;
				self.computer_turn();

				self.reset_moves();
				true
			},
			None => false,
		}
	}

	fn reset_moves(&mut self) {
		self.moves = self.gen.moves(self.state);
		self.reset_possible();
	}

	fn reset_possible(&mut self) {
		self.selected_passive = None; 
		self.selected_active = None;
		self.possible = self.moves.iter().map(|mv| mv.passive.from()).collect();
	}
}

#[derive(PartialEq, Properties)]
struct AppConfig {
	players: PlayerConfig,
	ended: Callback<()>,
}

impl Component for App {
	type Message = Msg;
	type Properties = AppConfig;

	fn create(ctx: &Context<Self>) -> Self {
		let mut app = Self {
			state: GameState::initial(),
			gen: MoveGen::new(),
			players: ctx.props().players.get(),
			surrendered: None,
			possible: vec![],
			moves: vec![],
			selected_passive: None,
			selected_active: None,
			history: vec![],
			last_move: None,
		};
		app.reset();
		app
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Select(sq) => {
				if self.players.computer_only() {
					false
				} else if self.selected_passive.is_none() && self.possible.contains(&sq) {
					self.selected_passive = Some(sq);
					self.possible = self.moves.iter()
						.filter(|mv| mv.passive.from() == sq)
						.map(|mv| mv.active.from()).collect();
					true
				} else if self.selected_active.is_none() && self.possible.contains(&sq) {
					self.selected_active = Some(sq);
					self.possible = self.moves.iter()
						.filter(|mv| mv.active.from() == sq && Some(mv.passive.from()) == self.selected_passive)
						.flat_map(|mv| [mv.passive.to(), mv.active.to()]).collect();
					true
				} else if self.selected_passive.is_some() && self.selected_active.is_some() && self.possible.contains(&sq) {
					match self.moves.iter().find(|mv| 
						Some(mv.passive.from()) == self.selected_passive && 
						Some(mv.active.from()) == self.selected_active &&
						(mv.passive.to() == sq || mv.active.to() == sq)
					) {
						Some(mv) => self.human_turn(*mv),
						None     => false,
					}
				} else {
					false
				}
			},
			Msg::Deselect => {
				self.reset_possible();
				true
			},
			Msg::Surrender => { self.surrendered = Some(self.state.current_side); true },

			Msg::Undo => self.undo(),
			Msg::Next => { self.history.push(self.state); self.computer_turn(); true },
			Msg::Reset => { self.reset(); true },
		}
	}

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
    	self.players = ctx.props().players.get();
    	self.reset();
    	true
    }

	fn view(&self, ctx: &Context<Self>) -> Html {
		let black_piece_url = "assets/samurai-helmet.svg";
		let white_piece_url = "assets/wolf-head.svg";

		let select = |sq| {
			if self.possible.contains(&sq) {
				ctx.link().callback(move |_| Msg::Select(sq))
			} else {
				ctx.link().callback(move |_| Msg::Deselect)
			}
		};

		let tiles = Board::iter().flat_map(|board| {
			(0..4).flat_map(move |row| {
				(0..4).map(move |col| {
					let sq = Square::from_local(board, row, col).unwrap();

					let x = 0.25 + board.coord().1 as f32*4.5 + col as f32;
					let y = 0.25 + board.coord().0 as f32*4.5 + row as f32;

					let cls = match board {
						Board::TopLeft      => "tl",
						Board::TopRight     => "tr",
						Board::BottomLeft   => "bl",
						Board::BottomRight  => "br",
					};

					html! {
						<rect 
							onclick={select(sq)}
							class={classes!(
								"tile", 
								cls,
								if self.selected_passive == Some(sq) || self.selected_active == Some(sq) {
									"selected"
								} else if self.possible.contains(&sq) && !self.players.computer_only() {
									"possible"
								} else { "" },
							)} 
							x={x.to_string()} y={y.to_string()} 
							width=1 height=1>
						</rect>
					}
				})
			})
		}).collect::<Html>();

		let pieces = logic::board::Board::iter().flat_map(|board| {
			(0..4).flat_map(move |row| {
				(0..4).map(move |col| {
					let sq = Square::from_local(board, row, col).unwrap();

					let x = 0.35 + board.coord().1 as f32*4.5 + col as f32;
					let y = 0.35 + board.coord().0 as f32*4.5 + row as f32;

					if self.state.pieces().get(sq) {
						let image = if self.state.blacks.get(sq) {
							black_piece_url
						} else {
							white_piece_url
						};

						html! {
							<image onclick={select(sq)} x={x.to_string()} y={y.to_string()} width=0.8 height=0.8 href={image}></image>
						}
					} else {
						html!(<></>)
					}
				})
			})
		}).collect::<Html>();

		let popup = if let Some(w) = self.state.winner().or(self.surrendered.map(|x| !x)) {
			html! {
				<div class="popup">
					<p>{
						match w {
							Color::Black => "Black wins.",
							Color::White => "White wins.",
						}
					}</p>
					<button onclick={let ended = ctx.props().ended.clone(); Callback::from(move |_| ended.emit(()))}>{"Play again"}</button>
				</div>
			}
		} else {
			html!(<></>)
		};

		let arrow = if let Some(mv) = self.last_move {
			let get_pos = |sq: Square| (
				0.75 + sq.board().coord().1 as f32*4.5 + sq.col() as f32, 
				0.75 + sq.board().coord().0 as f32*4.5 + sq.row() as f32
			);
			let dir = |(x0, y0): (f32, f32), (x1, y1): (f32, f32)| {
				let n = ((x1-x0).powf(2.)+(y1-y0).powf(2.)).sqrt();
				((x1-x0)/n, (y1-y0)/n)
			};

			let pf = get_pos(mv.passive.from());
			let _pt = get_pos(mv.passive.to());
			let pt = (_pt.0 - dir(pf, _pt).0*0.5, _pt.1 - dir(pf, _pt).1*0.5);

			let af = get_pos(mv.active.from());
			let _at = get_pos(mv.active.to());
			let at = (_at.0 - dir(af, _at).0*0.5, _at.1 - dir(af, _at).1*0.5);

			html! {
				<>
					<line class="arrow" style="pointer-events: none;" marker-end="url(#arrow)" stroke-width="0.1" x1={pf.0.to_string()} y1={pf.1.to_string()} x2={pt.0.to_string()} y2={pt.1.to_string()}></line>
					<line class="arrow" style="pointer-events: none;" marker-end="url(#arrow)" stroke-width="0.1" x1={af.0.to_string()} y1={af.1.to_string()} x2={at.0.to_string()} y2={at.1.to_string()}></line>
				</>
			}
		} else {
			html!(<></>)
		};

		html! {
			<>
				{popup}
				<div id="sidebar">
					<img src={if self.state.current_side == logic::game::Color::Black { black_piece_url } else { white_piece_url }} />
					<button class="time" onclick={ctx.link().callback(move |_| Msg::Undo)}>{"Back ←"}</button>
					if self.players.computer_only() {
						<button class="time" onclick={ctx.link().callback(move |_| Msg::Next)}>{"Next →"}</button>
					} else {
						<button onclick={ctx.link().callback(move |_| Msg::Surrender)}>{"Surrender"}</button>
					}
				</div>
				<svg id="shobu" viewBox="0 0 9 9" xmlns="http://www.w3.org/2000/svg" >
					<defs>
						<marker
							id="arrow"
							viewBox="0 0 10 10"
							refX="5"
							refY="5"
							markerWidth="4"
							markerHeight="4"
							orient="auto-start-reverse">
							<path class="arrow" d="M 0 0 L 10 5 L 0 10 z" />
						</marker>
					</defs>

					<rect x=0.25 y=0.25 width=4 height=4 class="board tl"></rect>
					<image x=0.25 y=0.25 width=4 height=4 href="assets/dark.jpeg"></image>
					<rect x=4.75 y=0.25 width=4 height=4 class="board tr"></rect>
					<image x=4.75 y=0.25 width=4 height=4 href="assets/light.jpeg"></image>

					<image x=0 y=4.3 width=9 href="assets/rope.png"></image>

					<rect x=0.25 y=4.75 width=4 height=4 class="board bl"></rect>
					<image x=0.25 y=4.75 width=4 height=4 href="assets/dark.jpeg"></image>
					<rect x=4.75 y=4.75 width=4 height=4 class="board br"></rect>
					<image x=4.75 y=4.75 width=4 height=4 href="assets/light.jpeg"></image>
					{tiles}
					{pieces}
					{arrow}

					<rect x=0 y=0 width=9 height=4.5 class="area horizontal"></rect>
					<rect x=0 y=4.5 width=9 height=4.5 class="area horizontal"></rect>
					<rect x=0 y=0 width=4.5 height=9 class="area vertical"></rect>
					<rect x=4.5 y=0 width=4.5 height=9 class="area vertical"></rect>
				</svg>
			</>
		}
	}
}

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
enum PlayerChoice {
	Human,
	RandomAI,
	MinimaxAI1,
	MinimaxAI2,
	MinimaxAI3,
}

impl PlayerChoice {
	fn get(&self) -> Player {
		use logic::control::agent::*;
		use logic::control::evaluation::*;

		match self {
			Self::Human => Player::Human,
			Self::RandomAI => Player::Computer(Box::new(RandomAgent)),
			Self::MinimaxAI1 => Player::Computer(Box::new(MinimaxAgent::new(1, simple))),
			Self::MinimaxAI2 => Player::Computer(Box::new(MinimaxAgent::new(2, simple))),
			Self::MinimaxAI3 => Player::Computer(Box::new(MinimaxAgent::new(3, simple))),
		}
	}

	fn name(&self) -> &'static str {
		match self {
			Self::Human => "Human",
			Self::RandomAI => "Random AI",
			Self::MinimaxAI1 => "Minimax AI, depth 1",
			Self::MinimaxAI2 => "Minimax AI, depth 2",
			Self::MinimaxAI3 => "Minimax AI, depth 3",
		}
	}
}

use std::fmt;

impl fmt::Display for PlayerChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
struct PlayerConfig {
	white: PlayerChoice,
	black: PlayerChoice,
}

impl PlayerConfig {
	fn get(&self) -> Players {
		Players {
			white: self.white.get(),
			black: self.black.get(),
		}
	}

	fn with_changed_black(&self, black: PlayerChoice) -> PlayerConfig {
		PlayerConfig {
			black,
			white: self.white,
		}
	}

	fn with_changed_white(&self, white: PlayerChoice) -> PlayerConfig {
		PlayerConfig {
			black: self.black,
			white
		}
	}
}

#[function_component]
fn GameSetup() -> Html {
	let players = use_state(|| PlayerConfig {
		black: PlayerChoice::MinimaxAI2,
		white: PlayerChoice::Human,
	});
	let shown = use_state(|| true);

	let options = vec![
		PlayerChoice::Human,
		PlayerChoice::RandomAI,
		PlayerChoice::MinimaxAI1,
		PlayerChoice::MinimaxAI2,
		PlayerChoice::MinimaxAI3,
	];

	let on_select_black = {
		let players = players.clone();
		Callback::from(move |s| players.set(players.with_changed_black(s)))
	};
	let on_select_white = {
		let players = players.clone();
		Callback::from(move |s| players.set(players.with_changed_white(s)))
	};

	html! {
		<>
			<div class="popup" hidden={!*shown}>
				<p>{"Welcome to the game of "}<strong class="shobu-text">{"SHŌBU"}</strong>{"! Please select your game configuration:"}</p>
				<table>
					<tr>
						<td><label for="black">{"Black: "}</label></td>
						<td><Select<PlayerChoice> name="black" default={players.black} options={options.clone()} on_select={on_select_black} /></td>
					</tr>
					<tr>
						<td><label for="white">{"White: "}</label></td>
						<td><Select<PlayerChoice> name="white" default={players.white} options={options} on_select={on_select_white} /></td>
					</tr>
					<tr>
						<td colspan=2><button onclick={ let shown = shown.clone(); Callback::from(move |_| shown.set(false)) }>{"Start"}</button></td>
					</tr>
				</table>

				<details>
					<summary>{"About"}</summary>
					<p>
						{"The game "}<strong class="shobu-text">{"SHŌBU"}</strong>{" was designed by "}<a href="https://www.smirkanddagger.com/product-page/shobu">{"Manolis Vranas and Jamie Sajdak, published by Smirk & Dagger games"}</a>
						{". You can find the rules "}<a target="_blank" href="https://drive.google.com/file/d/1L4h2-kT377Hfon0V4QUK42x2k0xMUxUc/view">{"here"}</a>{"."}
					</p>
					<p>
						{"This online version was created by "}<a href="https://veskrna.matfyz.cz">{"Lukáš Veškrna"}</a>{"."}
					</p>
				</details>
				<details>
					<summary>{"The controls"}</summary>
					<p>{"Every turn: "}</p>
					<ul>
						<li>{"Select the piece with which you will play your "}<strong>{"passive"}</strong>{" move."}</li>
						<li>{"Select the piece with which you will play your "}<strong>{"aggressive"}</strong>{" move."}</li>
						<li>{"Choose a valid move pair from the available highlighted options."}</li>
					</ul>
				</details>
				<details>
					<summary>{"Other mentions"}</summary>
					<p>{"Icons by "}<a href="https://game-icons.net/">{"https://game-icons.net/"}</a>{"."}</p>
				</details>
			</div>
			<div class={classes!(if *shown {"disabled"} else {""})}>
				<App players={*players} ended={Callback::from(move |_| shown.set(true))}/>
			</div>
		</>
	}
}


fn main() {
	yew::Renderer::<GameSetup>::new().render();
}
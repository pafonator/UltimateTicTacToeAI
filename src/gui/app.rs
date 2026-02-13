use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    X,
    O,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn to_string(self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellState {
    Empty,
    Occupied(Player),
}

#[derive(Clone, PartialEq, Debug)]
struct SmallBoard {
    cells: [CellState; 9],
    winner: Option<Player>,
}

impl SmallBoard {
    fn new() -> Self {
        Self {
            cells: [CellState::Empty; 9],
            winner: None,
        }
    }

    fn make_move(&mut self, index: usize, player: Player) -> bool {
        if self.winner.is_some() || self.cells[index] != CellState::Empty {
            return false;
        }
        self.cells[index] = CellState::Occupied(player);
        self.check_winner();
        true
    }

    fn check_winner(&mut self) {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
            [0, 4, 8], [2, 4, 6],            // diagonals
        ];

        for line in lines.iter() {
            if let CellState::Occupied(player) = self.cells[line[0]] {
                if self.cells[line[1]] == CellState::Occupied(player)
                    && self.cells[line[2]] == CellState::Occupied(player)
                {
                    self.winner = Some(player);
                    return;
                }
            }
        }
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&c| c != CellState::Empty)
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (boards, set_boards) = create_signal(vec![SmallBoard::new(); 9]);
    let (current_player, set_current_player) = create_signal(Player::X);
    let (active_board, set_active_board) = create_signal(None::<usize>);
    let (game_winner, set_game_winner) = create_signal(None::<Player>);

    let check_game_winner = move || {
        let boards_state = boards.get();
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
            [0, 4, 8], [2, 4, 6],            // diagonals
        ];

        for line in lines.iter() {
            if let Some(player) = boards_state[line[0]].winner {
                if boards_state[line[1]].winner == Some(player)
                    && boards_state[line[2]].winner == Some(player)
                {
                    set_game_winner.set(Some(player));
                    return;
                }
            }
        }
    };

    let handle_cell_click = move |board_idx: usize, cell_idx: usize| {
        if game_winner.get().is_some() {
            return;
        }

        // Check if this board can be played
        if let Some(active) = active_board.get() {
            if active != board_idx {
                return;
            }
        }

        let mut boards_state = boards.get();
        let player = current_player.get();

        if boards_state[board_idx].make_move(cell_idx, player) {
            set_boards.set(boards_state.clone());
            
            check_game_winner();
            
            if game_winner.get().is_none() {
                set_current_player.set(player.other());
                
                // Set next active board
                if boards_state[cell_idx].winner.is_none() && !boards_state[cell_idx].is_full() {
                    set_active_board.set(Some(cell_idx));
                } else {
                    set_active_board.set(None);
                }
            }
        }
    };

    let reset_game = move |_| {
        set_boards.set(vec![SmallBoard::new(); 9]);
        set_current_player.set(Player::X);
        set_active_board.set(None);
        set_game_winner.set(None);
    };

    let ai_play = move |_| {
        // TODO: Implement AI play
        println!("AI play");
    };

    view! {
        <div class="app">
            <h1>"Ultimate Tic Tac Toe"</h1>
            
            <div class="game-info">
                {move || {
                    if let Some(winner) = game_winner.get() {
                        view! { <h2 class="winner">"🎉 Player " {winner.to_string()} " wins! 🎉"</h2> }.into_view()
                    } else {
                        view! { <h2>"Current Player: " {current_player.get().to_string()}</h2> }.into_view()
                    }
                }}
            </div>

            <div class="mega-board">
                {(0..9).map(|board_idx| {
                    view! {
                        <div class={move || {
                            let mut classes = vec!["small-board"];
                            if boards.get()[board_idx].winner.is_some() {
                                classes.push("won");
                            }
                            if let Some(active) = active_board.get() {
                                if active == board_idx {
                                    classes.push("active");
                                }
                            } else if boards.get()[board_idx].winner.is_none() && !boards.get()[board_idx].is_full() {
                                classes.push("playable");
                            }
                            classes.join(" ")
                        }}>
                            {move || {
                                if let Some(winner) = boards.get()[board_idx].winner {
                                    view! {
                                        <div class="board-winner">{winner.to_string()}</div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="cells">
                                            {(0..9).map(|cell_idx| {
                                                view! {
                                                    <button
                                                        class="cell"
                                                        on:click=move |_| handle_cell_click(board_idx, cell_idx)
                                                    >
                                                        {move || {
                                                            match boards.get()[board_idx].cells[cell_idx] {
                                                                CellState::Empty => "".to_string(),
                                                                CellState::Occupied(player) => player.to_string().to_string(),
                                                            }
                                                        }}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_view()
                                }
                            }}
                        </div>
                    }
                }).collect_view()}
            </div>
            
            <button class="reset-button" on:click=reset_game>"New Game"</button>

            <button class="ai-play-button" on:click=ai_play>"AI Play"</button>

            <div class="rules">
                <h3>"How to Play"</h3>
                <p>"Win three small boards in a row to win the game!"</p>
                <p>"Your move determines which board your opponent plays next."</p>
                <p>"🟢 Green border = you must play here"</p>
                <p>"🔵 Blue border = you can play here"</p>
            </div>
        </div>
    }
}

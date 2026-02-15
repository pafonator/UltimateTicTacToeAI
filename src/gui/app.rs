use std::time::Duration;

use log::{info, warn};
use leptos::{*};
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::sleep;
use web_sys;
use gloo_worker::Spawnable;
use web_sys::window;

use crate::gui::ai_worker::{AIWorker, AIRequest, AIResponse};
use crate::gui::{controller::*};

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Player {
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

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum AIMode {
    OnCommand,
    PlayAsX,
    PlayAsO,
    PlayAsBoth,
}

impl AIMode {
    fn should_play(self, player: Player) -> bool {
        match self {
            AIMode::OnCommand => false,
            AIMode::PlayAsX => player == Player::X,
            AIMode::PlayAsO => player == Player::O,
            AIMode::PlayAsBoth => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CellState {
    Empty,
    Occupied(Player),
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct SmallBoard {
    pub cells: [CellState; 9],
    pub winner: Option<Player>,
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

    // AI timing controls
    let (ai_seconds, set_ai_seconds) = create_signal(5usize);
    let (ai_progress, set_ai_progress) = create_signal(0.0f64);
    let (ai_running, set_ai_running) = create_signal(false);
    let (ai_mode, set_ai_mode) = create_signal(AIMode::PlayAsO);

    let check_game_winner = move |use_untracked: bool| {
        let boards_state = if use_untracked {
            boards.get_untracked()
        } else {
            boards.get()
        };
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

    // Common function to make a move (works in both reactive and non-reactive contexts)
    let make_move = move |board_idx: usize, cell_idx: usize, use_untracked: bool| -> bool {
        // Check if game is over
        let is_game_over = if use_untracked {
            game_winner.get_untracked().is_some()
        } else {
            game_winner.get().is_some()
        };
        
        if is_game_over {
            warn!("Ignoring click on board {}, game is over", board_idx);
            return false;
        }

        // Check if this board can be played
        let current_active = if use_untracked {
            active_board.get_untracked()
        } else {
            active_board.get()
        };
        
        if let Some(active) = current_active {
            if active != board_idx {
                warn!("Ignoring click on board {}, active board is {}", board_idx, active);
                return false;
            }
        }

        let mut boards_state = if use_untracked {
            boards.get_untracked()
        } else {
            boards.get()
        };
        
        let player = if use_untracked {
            current_player.get_untracked()
        } else {
            current_player.get()
        };

        if boards_state[board_idx].make_move(cell_idx, player) {
            set_boards.set(boards_state.clone());
            
            if use_untracked  {
                check_game_winner(true);
            } else {
                check_game_winner(false);
            }
            
            let is_game_over_after = if use_untracked {
                game_winner.get_untracked().is_some()
            } else {
                game_winner.get().is_some()
            };
            
            if !is_game_over_after {
                set_current_player.set(player.other());
                
                // Set next active board
                if boards_state[cell_idx].winner.is_none() && !boards_state[cell_idx].is_full() {
                    set_active_board.set(Some(cell_idx));
                } else {
                    set_active_board.set(None);
                }
            }
            true
        } else {
            warn!("Ignoring click on board {}, cell {}", board_idx, cell_idx);
            false
        }
    };

    let handle_cell_click = move |board_idx: usize, cell_idx: usize| {
        if ai_running.get() {
            warn!("Ignoring click while AI is running");
            return;
        }
        make_move(board_idx, cell_idx, false);
    };

    let reset_game = move |_| {
        set_boards.set(vec![SmallBoard::new(); 9]);
        set_current_player.set(Player::X);
        set_active_board.set(None);
        set_game_winner.set(None);
    };

    // Create AI worker bridge
    let ai_worker = AIWorker::spawner()
        .callback(move |response: AIResponse| {
            if let Some((board_idx, cell_idx)) = response.best_move {
                // Apply the AI's move using the common make_move function
                info!("Applying AI move: {:?}", (board_idx, cell_idx));
                if !make_move(board_idx, cell_idx, true) {
                    warn!("AI move was invalid");
                }
            } else {
                warn!("AI failed to find a move");
            }
            set_ai_running.set(false);
            set_ai_progress.set(0.0);
        })
        .spawn("./ai_worker.js");

    // Signal to trigger AI computation
    let (ai_trigger, set_ai_trigger) = create_signal(0u32);

    // Effect to start AI computation when triggered
    create_effect(move |_| {
        // Watch the trigger signal
        let trigger_value = ai_trigger.get();
        
        // Skip the initial run (trigger starts at 0)
        if trigger_value == 0 {
            return;
        }
        
        // Check if we should actually run
        if ai_running.get_untracked() || game_winner.get_untracked().is_some() {
            return;
        }
        
        set_ai_running.set(true);
        set_ai_progress.set(0.0);
        let seconds = ai_seconds.get_untracked();
        
        // Capture current game state
        let boards_state = boards.get_untracked();
        let player = current_player.get_untracked();
        let active = active_board.get_untracked();

        ai_worker.send(AIRequest {
            state: grid_to_utttstate(&boards_state, player, active),
            seconds: seconds as u64,
        });
        
        spawn_local(async move {
            // Use web_sys's performance API instead of std::time::Instant (not available in WASM)
            let window = window().expect("no global `window` exists");
            let performance = window.performance().expect("performance should be available");
            let start_time = performance.now();
            let mut elapsed = (performance.now() - start_time) / 1000.0;
            let mut percent;
            // Start progress animation
            while ai_running.get_untracked()  {
                percent = elapsed / (seconds as f64); if percent > 1.0 { percent = 1.0; }
                set_ai_progress.set(percent);
                sleep(Duration::from_millis(200)).await;
                elapsed = (performance.now() - start_time) / 1000.0;
            }
        });
    });

    // Effect to automatically trigger AI based on mode
    create_effect(move |_| {
        let mode = ai_mode.get();
        let player = current_player.get();
        let is_game_over = game_winner.get().is_some();
        let is_ai_running = ai_running.get();
        
        if !is_game_over && !is_ai_running && mode.should_play(player) {
            // Small delay to let the UI update before AI plays
            spawn_local(async move {
                sleep(Duration::from_millis(300)).await;
                
                // Re-check conditions after delay and trigger AI
                if !ai_running.get_untracked() && !game_winner.get_untracked().is_some() {
                    set_ai_trigger.update(|v| *v = v.wrapping_add(1));
                }
            });
        }
    });

    let ai_play = move |_| {
        set_ai_trigger.update(|v| *v = v.wrapping_add(1));
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

            <div class="ai-progress-container">
                {move || {
                    if ai_running.get() {
                        view! {
                            <div class="progress-wrapper">
                                <div class="progress-bar" style={move || format!("width: {}%;", ai_progress.get() * 100.0)}></div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div class="progress-wrapper empty"></div> }.into_view()
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

            <div class="ai-controls">
                <div class="ai-input">
                    <span class="text">"AI Timeout"</span>
                    <input
                        type="number"
                        min="1"
                        max="60"
                        value={move || ai_seconds.get().to_string()}
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(v) = value.parse::<usize>() {
                                set_ai_seconds.set(v);
                            }
                        }
                    />
                    <span class="seconds-label">s</span>
                </div>
                
                <select 
                    class="ai-mode-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let mode = match value.as_str() {
                            "play_x" => AIMode::PlayAsX,
                            "play_o" => AIMode::PlayAsO,
                            "play_both" => AIMode::PlayAsBoth,
                            _ => AIMode::OnCommand,
                        };
                        set_ai_mode.set(mode);
                    }
                >
                    <option value="on_command" selected={move || ai_mode.get() == AIMode::OnCommand}>
                        "AI Run On Command"
                    </option>
                    <option value="play_x" selected={move || ai_mode.get() == AIMode::PlayAsX}>
                        "AI Play as X"
                    </option>
                    <option value="play_o" selected={move || ai_mode.get() == AIMode::PlayAsO}>
                        "AI Play as O"
                    </option>
                    <option value="play_both" selected={move || ai_mode.get() == AIMode::PlayAsBoth}>
                        "AI Play as X & O"
                    </option>
                </select>
                
                <button 
                    class="ai-play-button" 
                    on:click=ai_play 
                    disabled={move || ai_running.get() || ai_mode.get() != AIMode::OnCommand}
                >
                    {move || if ai_running.get() { "Running..." } else { "AI Play" }}
                </button>
            </div>

            <div class="rules">
                <h3>"How to Play"</h3>
                <p>"Win three small boards in a row to win the game!"</p>
                <p>"Your move determines which board your opponent plays next."</p>
                <p>"🔵 Blue border = you can play here"</p>
                <p>"Select AI mode from dropdown to enable automatic play."</p>
            </div>
        </div>
    }
}
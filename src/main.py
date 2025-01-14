import json
import threading
import tkinter as tk
from tkinter import messagebox
import subprocess
import time

class UltraGridApp:
    def __init__(self, player1_type="Human", player2_type="AI"):
        self.grid_data = self.initialize_empty_grid()
        self.player_types = {"X": player1_type, "O": player2_type}
        self.root = tk.Tk()
        self.root.title("Ultimate Tic Tac Toe")
        self.game_over = False  # Flag to stop the game when there's a winner
        self.create_gui()

    def current_player(self):
        return "X" if self.grid_data["crosses_turn"] else "O"

    @staticmethod
    def initialize_empty_grid():
        """Initialize an empty UltraGrid without `winner` attributes."""
        return {
            "ultra_grid": {
                "grid": [
                    [
                        {"grid": [["Empty"] * 3 for _ in range(3)]}
                        for _ in range(3)
                    ]
                    for _ in range(3)
                ]
            },
            "crosses_turn": True,
            "current_play_slot": 9
        }

    def check_winner(self, small_board):
        """Dynamically check if a grid has a winner."""
        grid = small_board["grid"]

        # Check rows and columns
        for i in range(3):
            if grid[i][0] == grid[i][1] == grid[i][2] != "Empty":
                return grid[i][0]
            if grid[0][i] == grid[1][i] == grid[2][i] != "Empty":
                return grid[0][i]

        # Check diagonals
        if grid[0][0] == grid[1][1] == grid[2][2] != "Empty":
            return grid[0][0]
        if grid[0][2] == grid[1][1] == grid[2][0] != "Empty":
            return grid[0][2]

        return "Empty"

    def check_big_grid_winner(self):
        """Dynamically check if the big grid has a winner."""
        big_grid = [
            [self.check_winner(small_board) for small_board in row]
            for row in self.grid_data["ultra_grid"]["grid"]
        ]

        # Check rows and columns
        for i in range(3):
            if big_grid[i][0] == big_grid[i][1] == big_grid[i][2] != "Empty":
                return big_grid[i][0]
            if big_grid[0][i] == big_grid[1][i] == big_grid[2][i] != "Empty":
                return big_grid[0][i]

        # Check diagonals
        if big_grid[0][0] == big_grid[1][1] == big_grid[2][2] != "Empty":
            return big_grid[0][0]
        if big_grid[0][2] == big_grid[1][1] == big_grid[2][0] != "Empty":
            return big_grid[0][2]

        return "Empty"

    def is_playable(self, board_x, board_y):
        """Check if a grid is playable (not won and has empty cells)."""
        small_board = self.grid_data["ultra_grid"]["grid"][board_x][board_y]
        if self.check_winner(small_board) != "Empty":
            return False
        for row in small_board["grid"]:
            if "Empty" in row:
                return True
        return False

    def apply_move(self, board_x, board_y, cell_x, cell_y):
        """Apply a move to the grid."""
        if self.game_over:
            messagebox.showerror("Error", f"Invalid move: The game is already over!")
            return  # Stop handling moves if the game is over

        move = f"Board ({board_x}, {board_y}), Cell ({cell_x}, {cell_y})"

        # Check if the move is valid for the current play slot
        if self.grid_data["current_play_slot"] != 9 and self.grid_data["current_play_slot"] != board_x * 3 + board_y:
            messagebox.showerror("Error", f"Invalid move: {move}. You must play in the current play slot!")
            return

        small_board = self.grid_data["ultra_grid"]["grid"][board_x][board_y]
        cell = small_board["grid"][cell_x][cell_y]

        # Check if the cell is already occupied
        if cell != "Empty":
            messagebox.showerror("Error", f"Invalid move: {move}. The cell is already occupied!")
            return

        # Update the grid
        small_board["grid"][cell_x][cell_y] = self.current_player()

        # Check for a winner in the big grid
        big_winner = self.check_big_grid_winner()
        if big_winner != "Empty":
            self.game_over = True
            self.refresh_gui()
            messagebox.showinfo("Game Over", f"{big_winner} wins the game!")
            return

        # Update the current play slot
        next_play_slot = cell_x * 3 + cell_y
        if self.is_playable(cell_x, cell_y):
            self.grid_data["current_play_slot"] = next_play_slot
        else:
            self.grid_data["current_play_slot"] = 9  # Reset to allow any playable slot if the target grid is unplayable

        # Switch the player
        self.grid_data["crosses_turn"] = not self.grid_data["crosses_turn"]

        # Handle AI Turn if Needed
        self.refresh_gui()
        self.handle_turn()

    def handle_turn(self):
        """Handle turns for Human and AI players."""
        if self.player_types[self.current_player()] == "AI":
            threading.Thread(target=self.simulate_ai_move).start()

    def simulate_ai_move(self):
        """Simulate an AI move by communicating with an external program."""
        try:
            timeout = 3
            print("Running AI for {} seconds".format(timeout))
            
            # Export current grid to JSON
            grid_json = json.dumps(self.grid_data)

            start_time = time.time()
            for d in range(5, 20):
                try:
                    remaining_time = timeout - (time.time() - start_time)
                    executable = "target/release/UltimateTicTacToe"
                    #print(f" - Running command:\n{executable} {d} '{grid_json}'")
                    # Run the external program and get its output
                    result = subprocess.run(
                        [executable, str(d) ,grid_json],
                        text=True,
                        capture_output=True,
                        check=True,
                        timeout= remaining_time,  # Remaining time
                    )
                    depth = d
                except subprocess.TimeoutExpired:
                    print(f"[Rust Output]:\nTimeout\n")
                    break
            print(f"[Rust Output] for depth {depth}:\n{result.stdout}\n")
            
            # Separate logs and result
            result_json = None
            for line in result.stdout.splitlines():
                if line.startswith("[RESULT]"):
                    result_json = line[len("[RESULT] "):]  # Extract the JSON part

            # Parse the move from the output
            move = json.loads(result_json)
            board_x, board_y = move[0] // 3, move[0] % 3
            cell_x, cell_y = move[1] // 3, move[1] % 3

            # Apply the move
            self.apply_move(board_x, board_y, cell_x, cell_y)

        except json.JSONDecodeError:
            messagebox.showerror("Error", "Invalid move received from AI program!")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to run AI program: {e}")

    def refresh_gui(self):
        """Refresh the GUI to reflect updated game state."""
        for widget in self.root.winfo_children():
            widget.destroy()
        self.create_gui()

    def create_gui(self):
        """Create the Tkinter GUI to visualize the UltraGrid."""
        main_frame = tk.Frame(self.root, padx=10, pady=10)
        main_frame.pack()

        for board_x, row in enumerate(self.grid_data["ultra_grid"]["grid"]):
            for board_y, small_board in enumerate(row):
                # Dynamically determine the winner of this small board
                winner = self.check_winner(small_board)

                # Create a frame for each small board
                board_frame = tk.Frame(main_frame, borderwidth=2, relief="solid", padx=5, pady=5)
                board_frame.grid(row=board_x, column=board_y, padx=5, pady=5)

                if winner != "Empty":
                    # Display the winner as a giant cell
                    tk.Label(
                        board_frame,
                        text=winner,
                        font=("Arial", 18, "bold"),
                        bg="lightgray",
                        width=6,
                        height=3,
                        borderwidth=2,
                        relief="solid",
                    ).pack()
                else:
                    for cell_x, cell_row in enumerate(small_board["grid"]):
                        for cell_y, cell in enumerate(cell_row):
                            # Determine text and color for the cell
                            button_text = " "
                            fg_color = "black"  # Default text color
                            if cell == "X":
                                button_text = "X"
                                fg_color = "red"
                            elif cell == "O":
                                button_text = "O"
                                fg_color = "blue"

                            # Use Label for disabled cells to preserve colors
                            if (
                                self.game_over
                                or self.player_types[self.current_player()] == "AI"
                                or (self.grid_data["current_play_slot"] != 9 and self.grid_data["current_play_slot"] != board_x * 3 + board_y)
                            ):
                                tk.Label(
                                    board_frame,
                                    text=button_text,
                                    width=2,
                                    height=1,
                                    font=("Arial", 12),
                                    fg=fg_color,
                                    bg="#d3d3d3",  # Disabled background color
                                    borderwidth=1,
                                    relief="solid",
                                ).grid(row=cell_x, column=cell_y, padx=2, pady=2)
                            else:
                                # Use Button for interactive cells
                                tk.Button(
                                    board_frame,
                                    text=button_text,
                                    width=2,
                                    height=1,
                                    font=("Arial", 12),
                                    fg=fg_color,
                                    bg="white",
                                    command=lambda bx=board_x, by=board_y, cx=cell_x, cy=cell_y: self.apply_move(bx, by, cx, cy),
                                ).grid(row=cell_x, column=cell_y, padx=2, pady=2)

    def run(self):
        """Run the Tkinter main loop."""
        self.handle_turn()
        self.root.mainloop()


def main():
    app = UltraGridApp(player1_type="Human", player2_type="AI")  
    app.run()


if __name__ == "__main__":
    main()

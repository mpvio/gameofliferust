Source for patterns: https://conwaylife.com/wiki/Main_Page

# Key Points
1. Run with cargo run
2. custom_universe.rs takes a width and height as parameter to set as the size of the universe.
3. For each recursion of the mainloop and every cell within this universe, the alive/ dead states of neighbour cells are checked (count_live_neighbors). Then, each cell is set to alive or dead using the rules of the game (next_generation).
4. The universe is displayed in the terminal, which is cleared between each recursion.

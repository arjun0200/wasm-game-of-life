use wasm_game_of_life::*;
use std::time::Duration;
use std::thread;

fn main() {
    println!("🎮 WebAssembly Game of Life - Debug Mode");
    println!("=====================================");
    
    // Create a new universe
    let mut universe = Universe::new();
    
    // DEBUG: Add breakpoint here to see initial universe state
    let _debug_breakpoint_1 = (); // Set breakpoint on this line
    println!("Initial universe created!");
    
    // Print initial state
    print_universe(&universe, "Initial State");
    
    // Run simulation for a few generations
    for generation in 1..=10 {
        // DEBUG: Add breakpoint here to see each generation
        let _debug_breakpoint_2 = (); // Set breakpoint on this line
        
        println!("\n--- Generation {} ---", generation);
        universe.tick();
        
        // DEBUG: Add breakpoint here to see state after tick
        let _debug_breakpoint_3 = (); // Set breakpoint on this line
        
        // Print every 2nd generation to avoid too much output
        if generation % 2 == 0 {
            print_universe(&universe, &format!("Generation {}", generation));
        }
        
        // Add a small delay to see the evolution
        thread::sleep(Duration::from_millis(500));
    }
    
    // DEBUG: Add breakpoint here to see final state
    let _debug_breakpoint_4 = (); // Set breakpoint on this line
    println!("\n🎯 Final state:");
    print_universe(&universe, "Final State");
    
    // Demonstrate cell inspection
    demonstrate_cell_inspection(&universe);
    
    // Analyze the universe pattern
    analyze_universe_pattern(&universe);
    
    // Show game rules
    demonstrate_game_rules();
}

fn print_universe(universe: &Universe, title: &str) {
    println!("\n{}:", title);
    println!("{}", universe.render());
}

fn demonstrate_cell_inspection(universe: &Universe) {
    println!("\n🔍 Cell Inspection Demo:");
    println!("========================");
    
    // Since we can't access private fields directly, we'll work with the render output
    let rendered = universe.render();
    let lines: Vec<&str> = rendered.lines().collect();
    
    println!("Universe rendered as {} lines", lines.len());
    
    // Show first few lines
    for (i, line) in lines.iter().take(5).enumerate() {
        println!("Line {}: {}", i, line);
    }
    
    if lines.len() > 5 {
        println!("... (showing first 5 lines only)");
    }
}

// Helper functions for debugging - these work with the public API
fn analyze_universe_pattern(universe: &Universe) {
    println!("\n📊 Universe Analysis:");
    println!("====================");
    
    let rendered = universe.render();
    let lines: Vec<&str> = rendered.lines().collect();
    
    let mut alive_count = 0;
    let mut dead_count = 0;
    
    for line in &lines {
        for ch in line.chars() {
            match ch {
                '■' => alive_count += 1,
                '□' => dead_count += 1,
                _ => {} // Skip other characters
            }
        }
    }
    
    println!("Total cells: {}", alive_count + dead_count);
    println!("Alive cells: {}", alive_count);
    println!("Dead cells: {}", dead_count);
    println!("Alive percentage: {:.2}%", 
             (alive_count as f64 / (alive_count + dead_count) as f64) * 100.0);
}

fn demonstrate_game_rules() {
    println!("\n📋 Conway's Game of Life Rules:");
    println!("==============================");
    println!("1. Any live cell with fewer than two live neighbors dies (underpopulation)");
    println!("2. Any live cell with two or three live neighbors lives on to the next generation");
    println!("3. Any live cell with more than three live neighbors dies (overpopulation)");
    println!("4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction)");
    println!();
}

fn demonstrate_neighbor_algorithm() {
    println!("\n🔍 Neighbor Checking Algorithm Explained:");
    println!("=========================================");
    
    // Simulate a small 3x3 grid for demonstration
    let width = 3;
    let height = 3;
    let center_row = 1;
    let center_col = 1;
    
    println!("For a 3x3 grid, checking neighbors of cell at position (1,1):");
    println!("Grid layout:");
    println!("(0,0) (0,1) (0,2)");
    println!("(1,0) (1,1) (1,2)  <- (1,1) is the center cell");
    println!("(2,0) (2,1) (2,2)");
    println!();
    
    println!("The loop: for delta_row in [self.height - 1, 0, 1].iter().cloned()");
    println!("For height=3: [2, 0, 1]");
    println!("This creates these delta values:");
    println!("  delta_row = 2, delta_col = 2  -> neighbor at (1+2, 1+2) = (3,3) -> wraps to (0,0)");
    println!("  delta_row = 2, delta_col = 0  -> neighbor at (1+2, 1+0) = (3,1) -> wraps to (0,1)");
    println!("  delta_row = 2, delta_col = 1  -> neighbor at (1+2, 1+1) = (3,2) -> wraps to (0,2)");
    println!("  delta_row = 0, delta_col = 2  -> neighbor at (1+0, 1+2) = (1,3) -> wraps to (1,0)");
    println!("  delta_row = 0, delta_col = 0  -> SKIPPED (this is the center cell itself)");
    println!("  delta_row = 0, delta_col = 1  -> neighbor at (1+0, 1+1) = (1,2)");
    println!("  delta_row = 1, delta_col = 2  -> neighbor at (1+1, 1+2) = (2,3) -> wraps to (2,0)");
    println!("  delta_row = 1, delta_col = 0  -> neighbor at (1+1, 1+0) = (2,1)");
    println!("  delta_row = 1, delta_col = 1  -> neighbor at (1+1, 1+1) = (2,2)");
    println!();
    
    println!("The 8 neighbors checked are:");
    println!("(0,0) (0,1) (0,2)");
    println!("(1,0) [C]  (1,2)  <- [C] is center, others are neighbors");
    println!("(2,0) (2,1) (2,2)");
    println!();
    
    println!("Key points:");
    println!("1. [self.height - 1, 0, 1] creates offsets: [-1, 0, +1]");
    println!("2. The modulo operation (%) handles wrapping around the edges");
    println!("3. delta_row == 0 && delta_col == 0 is skipped (center cell)");
    println!("4. This checks all 8 surrounding cells in a 3x3 grid");
}

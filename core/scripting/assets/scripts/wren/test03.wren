// Conway's Game of Life
class GameOfLife {
  construct new(width, height) {
    _width = width
    _height = height
    _grid = List.filled(width * height, false)
  }

  // Get cell at x,y coordinates
  cell(x, y) {
    if (x < 0 || x >= _width || y < 0 || y >= _height) return false
    return _grid[y * _width + x]
  }

  // Set cell at x,y coordinates
  setCell(x, y, value) {
    if (x < 0 || x >= _width || y < 0 || y >= _height) return
    _grid[y * _width + x] = value
  }

  // Count live neighbors for cell at x,y
  countNeighbors(x, y) {
    var count = 0
    for (dy in -1..2) {
      for (dx in -1..2) {
        if (dx == 0 && dy == 0) continue
        if (cell(x + dx, y + dy)) count = count + 1
      }
    }
    return count
  }

  // Update the grid according to Game of Life rules
  step() {
    var newGrid = List.filled(_width * _height, false)

    for (y in 0..._height) {
      for (x in 0..._width) {
        var neighbors = countNeighbors(x, y)
        var alive = cell(x, y)

        if (alive && (neighbors == 2 || neighbors == 3)) {
          newGrid[y * _width + x] = true
        } else if (!alive && neighbors == 3) {
          newGrid[y * _width + x] = true
        }
      }
    }

    _grid = newGrid
  }

  // Display the current state
  display() {
    for (y in 0..._height) {
      var line = ""
      for (x in 0..._width) {
        line = line + (cell(x, y) ? "■ " : "□ ")
      }
      System.print(line)
    }
    System.print("")
  }
}

// Create a 10x10 game board
var game = GameOfLife.new(10, 10)

// Set up a simple glider pattern
game.setCell(1, 0, true)
game.setCell(2, 1, true)
game.setCell(0, 2, true)
game.setCell(1, 2, true)
game.setCell(2, 2, true)

// Run for 10 generations
for (i in 0...10) {
  System.print("Generation %(i):")
  game.display()
  game.step()
}

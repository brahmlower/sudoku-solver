
# Sudoku Solver

This is a poorly designed and unoptimized sudoku solver. It has no considerations for abstractions, reuse or efficiency. It was just a fun friday evening project 🤷‍♂️

To do:
- Implement wave form collapse to follow cell updates
- Easier loading/saving of sudoku boards
  - Loading new boards from an api would be great
- Possibly guessing when a cell can't be determined
  - Requires a historical log of changes so they can be reverted if a guess is incorrect

Running outputs the following:
```
Starting 🤖
┏━━━━━━━┳━━━━━━━┳━━━━━━━┓
┃ ⁹ 4 ⁹ ┃ 6 ⁹ 2 ┃ ⁹ 3 1 ┃
┃ ⁹ ⁹ ⁹ ┃ ⁹ ⁹ 1 ┃ 6 ⁹ 9 ┃
┃ 6 ⁹ ⁹ ┃ 5 4 ⁹ ┃ 8 2 7 ┃
┣━━━━━━━╋━━━━━━━╋━━━━━━━┫
┃ ⁹ ⁹ 2 ┃ 7 6 ⁹ ┃ ⁹ 8 ⁹ ┃
┃ 5 ⁹ 6 ┃ ⁹ ⁹ ⁹ ┃ ⁹ 7 4 ┃
┃ ⁹ 8 7 ┃ ⁹ ⁹ 5 ┃ ⁹ 6 2 ┃
┣━━━━━━━╋━━━━━━━╋━━━━━━━┫
┃ 1 6 ⁹ ┃ ⁹ 8 ⁹ ┃ ⁹ 5 ⁹ ┃
┃ 8 2 ⁹ ┃ ⁹ ⁹ 7 ┃ ⁹ 9 ⁹ ┃
┃ 7 ⁹ ⁹ ┃ ⁹ ⁹ 6 ┃ 2 ⁹ ⁹ ┃
┗━━━━━━━┻━━━━━━━┻━━━━━━━┛
Solved! 💪
┏━━━━━━━┳━━━━━━━┳━━━━━━━┓
┃ 9 4 8 ┃ 6 7 2 ┃ 5 3 1 ┃
┃ 2 7 5 ┃ 8 3 1 ┃ 6 4 9 ┃
┃ 6 3 1 ┃ 5 4 9 ┃ 8 2 7 ┃
┣━━━━━━━╋━━━━━━━╋━━━━━━━┫
┃ 4 9 2 ┃ 7 6 3 ┃ 1 8 5 ┃
┃ 5 1 6 ┃ 9 2 8 ┃ 3 7 4 ┃
┃ 3 8 7 ┃ 4 1 5 ┃ 9 6 2 ┃
┣━━━━━━━╋━━━━━━━╋━━━━━━━┫
┃ 1 6 9 ┃ 2 8 4 ┃ 7 5 3 ┃
┃ 8 2 3 ┃ 1 5 7 ┃ 4 9 6 ┃
┃ 7 5 4 ┃ 3 9 6 ┃ 2 1 8 ┃
┗━━━━━━━┻━━━━━━━┻━━━━━━━┛
```
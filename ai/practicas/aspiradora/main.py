from enum import Enum
from random import random, randint
import os
from time import sleep


class Tile(Enum):
    Clean = "⬜",
    Dirty = "🟫",
    Occupied = "🧹",


class Environment:
    """
    Manages the grid of tiles and gives information about it
    """

    def _random_grid(self, width: int, dirty_ratio: float = 0.2) -> None:
        """
        Create a random environment with the given width, height, and dirty ratio
        Arguments:
            width: The width of the environment
        """
        # creates a multidimensional array of width x height tiles where each one will be clean if after a random
        # throw from 0 to 1 it gets a value greater than the dirty ratio and dirty otherwise
        self.grid = [Tile.Clean if random() > dirty_ratio else Tile.Dirty for _ in range(width)]
        self._map = self.grid.copy()

    def __init__(self, width: int):
        self._random_grid(width, 0.4)

    def __repr__(self):
        return "".join(tile.value[0] for tile in self.grid)

    def __str__(self):
        return self.__repr__()

    def is_bound(self, x: int) -> bool:
        """
        Check if both x and y are within the grid
        """
        return 0 <= x < len(self.grid)

    def is_dirty(self, x: int) -> bool:
        """
        Check if the tile at (x, y) is dirty
        """
        return self.is_bound(x) and self._map[x] == Tile.Dirty

    def clean(self, x: int) -> None:
        """
        Clean the tile at (x, y)
        """
        if self.is_bound(x):
            self._map[x] = Tile.Clean

    def is_everything_clean(self) -> bool:
        for tile in self._map:
            if tile == Tile.Dirty:
                return False
        return True


class Cleaner:
    class Direction(Enum):
        Left = -1
        Right = 1

    def __init__(self, env: Environment, x: int):
        self.environment = env
        self.environment.grid[x] = Tile.Occupied
        self.x = x
        self.score = 0
        self.direction = Cleaner.Direction.Left

    def take_decision(self):
        if self.environment.is_dirty(self.x):
            self.score += 1
            self.environment.clean(self.x)
        else:
            self.move()

    def move(self):
        self.environment.grid[self.x] = Tile.Clean

        self.x += int(self.direction.value)

        if not self.environment.is_bound(self.x):
            if self.direction == Cleaner.Direction.Left:
                self.direction = Cleaner.Direction.Right
            elif self.direction == Cleaner.Direction.Right:
                self.direction = Cleaner.Direction.Left
            self.x += int(self.direction.value) * 2

        self.environment.grid[self.x] = Tile.Occupied


def clear():
    # for windows
    if os.name == 'nt':
        _ = os.system('cls')
    # for mac and linux(here, os.name is 'posix')
    else:
        _ = os.system('clear')


if __name__ == "__main__":
    SIZE = 10
    clear()
    environment = Environment(SIZE)
    cleaner = Cleaner(environment, randint(0, SIZE - 1))
    while not cleaner.environment.is_everything_clean():
        clear()
        print(environment)
        cleaner.take_decision()
        print(f'Score = {cleaner.score}')
        sleep(0.5)

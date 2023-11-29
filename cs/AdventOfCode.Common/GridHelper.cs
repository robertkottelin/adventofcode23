using System;

namespace AdventOfCode.Common
{
    public static class GridHelper
    {
        public static void PrintGrid<T>(T[,] grid)
        {
            int rows = grid.GetLength(0);
            int cols = grid.GetLength(1);
            for (int i = 0; i < rows; i++)
            {
                for (int j = 0; j < cols; j++)
                {
                    Console.Write(grid[i, j] + "\t");
                }
                Console.WriteLine();
            }
        }

        // Add other grid-related methods as needed.
    }
}

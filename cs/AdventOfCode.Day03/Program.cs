using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Program
{
    static void Main()
    {
        // Read the input from the file
        var input = File.ReadAllLines("data.txt");

        var partNumbers = new List<uint>();
        var gears = new Dictionary<(int, int), List<uint>>();

        for (int row = 0; row < input.Length; row++)
        {
            var line = input[row].ToCharArray();
            int col1 = 0;

            while (col1 < line.Length)
            {
                while (col1 < line.Length && !char.IsDigit(line[col1]))
                {
                    col1++;
                }

                if (col1 >= line.Length) break;

                int col2 = col1;
                while (col2 < line.Length && char.IsDigit(line[col2]))
                {
                    col2++;
                }

                // Number found
                var numberString = new String(line[col1..col2]);
                if (uint.TryParse(numberString, out uint number))
                {
                    int startRow = row > 0 ? row - 1 : 0;
                    int endRow = Math.Min(row + 2, input.Length);
                    int startCol = col1 > 0 ? col1 - 1 : 0;
                    int endCol = Math.Min(col2 + 1, line.Length);

                    for (int i = startRow; i < endRow; i++)
                    {
                        for (int j = startCol; j < endCol; j++)
                        {
                            if (!char.IsDigit(input[i][j]) && input[i][j] != '.')
                            {
                                if (input[i][j] == '*')
                                {
                                    if (!gears.ContainsKey((i, j)))
                                    {
                                        gears.Add((i, j), new List<uint>());
                                    }
                                    gears[(i, j)].Add(number);
                                }
                                partNumbers.Add(number);
                                goto NextNumber;
                            }
                        }
                    }
                }

                NextNumber:
                col1 = col2;
            }
        }

        uint part1 = (uint)partNumbers.Sum(x => (long)x);
        Console.WriteLine($"Part 1: {part1}");

        uint part2 = (uint)gears.Values.Where(g => g.Count == 2).Sum(g => g.Aggregate(1L, (a, b) => a * b));
        Console.WriteLine($"Part 2: {part2}");

    }
}

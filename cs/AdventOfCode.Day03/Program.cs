using System;

class Program
{
    static void Main()
    {
        string[] schematic = {
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        };

        int sum = SumNumbers(schematic);
        Console.WriteLine($"The sum of all part numbers is: {sum}");
    }

    static int SumNumbers(string[] schematic)
    {
        int sum = 0;

        for (int i = 0; i < schematic.Length; i++)
        {
            for (int j = 0; j < schematic[i].Length; j++)
            {
                if (Char.IsDigit(schematic[i][j]))
                {
                    string numberStr = ExtractNumber(schematic, i, j);
                    if (IsAdjacentToSymbol(schematic, i, j, numberStr.Length))
                    {
                        sum += int.Parse(numberStr);
                    }
                }
            }
        }
        return sum;
    }

    static string ExtractNumber(string[] schematic, int x, int y)
    {
        string numberStr = "";
        while (y < schematic[x].Length && Char.IsDigit(schematic[x][y]))
        {
            numberStr += schematic[x][y];
            y++;
        }
        return numberStr;
    }

    static bool IsAdjacentToSymbol(string[] schematic, int x, int y, int length)
    {
        for (int i = -1; i <= 1; i++)
        {
            for (int j = -1; j <= length; j++)
            {
                int newX = x + i;
                int newY = y + j - 1; // Adjust for the length of the number

                if (newX >= 0 && newX < schematic.Length && newY >= 0 && newY < schematic[newX].Length)
                {
                    if (!Char.IsDigit(schematic[newX][newY]) && schematic[newX][newY] != '.')
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

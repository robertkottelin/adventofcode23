using System;
using System.Collections.Generic;
using System.IO;

class Program
{
    static void Main()
    {
        string path = "C:/Users/rober/Documents/GitHub/adventofcode23/cs/AdventOfCode.Day02/data.txt"; 
        var maxCubes = (Red: 12, Green: 13, Blue: 14);
        int possibleGamesSum = 0;

        foreach (var line in File.ReadLines(path))
        {
            var gameData = ParseGame(line);
            if (gameData != null)
            {
                var (gameId, sets) = gameData.Value;
                bool isValid = true;
                foreach (var (red, green, blue) in sets)
                {
                    if (red > maxCubes.Red || green > maxCubes.Green || blue > maxCubes.Blue)
                    {
                        isValid = false;
                        break;
                    }
                }

                if (isValid)
                {
                    possibleGamesSum += gameId;
                }
            }
        }

        Console.WriteLine($"Sum of IDs of possible games: {possibleGamesSum}");
    }

    static (int GameId, List<(int Red, int Green, int Blue)> Sets)? ParseGame(string line)
    {
        var parts = line.Split(new[] { ": " }, StringSplitOptions.None);
        if (parts.Length != 2)
        {
            return null;
        }

        if (!int.TryParse(parts[0].Replace("Game ", ""), out int gameId))
        {
            return null;
        }

        var setsStr = parts[1].Split(new[] { "; " }, StringSplitOptions.None);
        var sets = new List<(int Red, int Green, int Blue)>();

        foreach (var setStr in setsStr)
        {
            int red = 0, green = 0, blue = 0;
            var cubeStrs = setStr.Split(new[] { ", " }, StringSplitOptions.None);
            foreach (var cubeStr in cubeStrs)
            {
                var cubeParts = cubeStr.Split(' ');
                if (cubeParts.Length != 2 || !int.TryParse(cubeParts[0], out int count))
                {
                    return null;
                }

                switch (cubeParts[1])
                {
                    case "red":
                        red = count;
                        break;
                    case "green":
                        green = count;
                        break;
                    case "blue":
                        blue = count;
                        break;
                    default:
                        return null;
                }
            }
            sets.Add((red, green, blue));
        }

        return (gameId, sets);
    }
}

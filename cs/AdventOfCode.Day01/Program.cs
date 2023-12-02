using AdventOfCode.Common;

var lines = FileHelper.ReadAllLines("data.txt");
// var numbers = ConvertHelper.ConvertLinesToInts(lines);

int sum = lines.Select(line => 
    {
        var chars = line.ToCharArray();
        var firstDigit = chars.FirstOrDefault(c => char.IsDigit(c));
        var lastDigit = chars.LastOrDefault(c => char.IsDigit(c));

        if (firstDigit != default(char) && lastDigit != default(char))
        {
            string value = firstDigit.ToString() + lastDigit.ToString();
            return int.TryParse(value, out int result) ? result : 0;
        }
        else
        {
            return 0;
        }
    }).Sum();

Console.WriteLine($"Total sum of calibration values: {sum}");

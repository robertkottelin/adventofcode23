using System;

namespace AdventOfCode.Common
{
    public static class StringHelper
    {
        public static string[] SplitByDelimiter(string input, char delimiter)
        {
            return input.Split(delimiter, StringSplitOptions.RemoveEmptyEntries);
        }

        // Add other string manipulation methods as needed.
    }
}

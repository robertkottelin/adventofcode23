using System;
using System.Linq;

namespace AdventOfCode.Common
{
    public static class ConvertHelper
    {
        public static int[] ConvertLinesToInts(string[] lines)
        {
            return lines.Select(int.Parse).ToArray();
        }
    }
}

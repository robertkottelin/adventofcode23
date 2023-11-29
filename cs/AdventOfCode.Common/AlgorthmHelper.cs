using System;

namespace AdventOfCode.Common
{
    public static class AlgorithmHelper
    {
        public static int BinarySearch<T>(T[] array, T value) where T : IComparable<T>
        {
            int low = 0, high = array.Length - 1;
            while (low <= high)
            {
                int mid = low + (high - low) / 2;
                int comparison = array[mid].CompareTo(value);
                if (comparison == 0) return mid;
                if (comparison < 0) low = mid + 1;
                else high = mid - 1;
            }
            return -1;
        }

        // Add other algorithms here
    }
}

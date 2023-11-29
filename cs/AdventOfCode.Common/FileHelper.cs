using System;
using System.IO;
using System.Linq;

namespace AdventOfCode.Common
{
    public static class FileHelper
    {
        public static string[] ReadAllLines(string filePath)
        {
            return File.ReadAllLines(filePath);
            
        }

        public static string ReadAllText(string filePath)
        {
            return File.ReadAllText(filePath);
        }
    }
}

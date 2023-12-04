using System.Text.RegularExpressions;
using System.Text;

// Fucking Hell, that was the hardest problem i've ever solved i honestly don't know how the fuck i managed to solve it
// it took me an unbelievable 10 hours and three drafts to get it right, phew.
class Day3
{
    /*public static void Main()
    {
        string txtPath = @"input3.txt";
        string lines = string.Join("", File.ReadAllText(txtPath).ToCharArray().Select(x => (x == '\r' || x == '\n') ? "" : x.ToString()).ToArray());
        int columns = File.ReadAllLines(txtPath)[0].Length;

        int i = -1;
        foreach (char character in lines)
        {
            i++;
            if (character == '.' || char.IsDigit(character))
                continue;

            if (((i % columns) > 1 || (i % columns) == 0) && char.IsDigit(lines.ElementAt(i - 1)))
            {
                string num = Regex.Match(lines[((i % columns) >= 4 ? i - 3 : ((int)Floor((float)i / columns) * columns))..i], @"\d+").Value;
                lines = lines.Replace(num + lines.ElementAt(i), (num.Length == 4 ? "...." : num.Length == 3 ? "..." : num.Length == 2 ? ".." : ".") + lines.ElementAt(i));
                Console.WriteLine(num + "-" + i);
            }
            if ((i % columns) != 0 && char.IsDigit(lines.ElementAt(i + 1)))
            {
                string num = Regex.Match(lines[(i + 1)..(((i % columns) <= (columns - 3)) ? i + 4 : columns)], @"\d+").Value;
                lines = lines.Replace(lines.ElementAt(i) + num, lines.ElementAt(i) + (num.Length == 4 ? "...." : num.Length == 3 ? "..." : num.Length == 2 ? ".." : "."));
                Console.WriteLine(num + "+" + i);
            }

            if (i > columns && char.IsDigit(lines.ElementAt(i - columns)))
            {
                //string num = Regex.Match(lines[(i >= 3 ? i - 3 : 0)..(i - columns)], @"\d+").Value;
                //lines = lines.Replace(num + lines.ElementAt(i), (num.Length == 4 ? "...." : num.Length == 3 ? "..." : num.Length == 2 ? ".." : ".") + lines.ElementAt(i));
                //Console.WriteLine(num + "-" + i);
            }
            //Console.WriteLine(Regex.Match(lines[i..(i + 1)], @"\d+").Value);
        }
    }*/

    public static void Start()
    {
        string txtPath = @"input3.txt";
        string[] lines = File.ReadAllLines(txtPath);

        Dictionary<int[], int> map = new();
        List<int> normalizedNumbers = new();

        int columns = lines[0].Length;
        int sum = 0;

        for (int i = 0; i < lines.Length; i++)
            for (int j = 0; j < lines[i].Length; j++)
                map.Add(new int[2] { i, j }, -1);

        for (int i = 0; i < lines.Length; i++)
        {
            string newLine = lines[i];
            while (int.TryParse(Regex.Match(newLine, @"\d+").Value, out int num))
            {
                int index = newLine.IndexOf(num.ToString());
                normalizedNumbers.Add(num);

                StringBuilder myStringBuilder = new(newLine);
                myStringBuilder.Replace(num.ToString(), string.Concat(Enumerable.Repeat("|", num.ToString().Length)), index, 3);
                newLine = myStringBuilder.ToString();

                for (int j = 0; j < columns; j++)
                {
                    if (newLine.ElementAt(j) == '|')
                    {
                        map.Remove(map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i, j })).Key);
                        map.Add(new int[2] { i, j }, normalizedNumbers.Count - 1);
                    }
                }

                myStringBuilder = new(newLine);
                myStringBuilder.Replace(string.Concat(Enumerable.Repeat("|", num.ToString().Length)), string.Concat(Enumerable.Repeat(".", num.ToString().Length)), index, 3);
                newLine = myStringBuilder.ToString();
            }
        }

        List<int> takenIndices = new();
        for (int i = 0; i < lines.Length; i++)
        {
            for (int j = 0; j < columns; j++)
            {
                char character = lines[i].ElementAt(j);
                if (character != '*')
                    continue;

                List<int> partNums = new();

                if (i < lines.Length - 1)
                {
                    int index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i + 1, j })).Value;

                    if (index != -1 && !takenIndices.Contains(index))
                    {
                        partNums.Add(normalizedNumbers[index]);
                        takenIndices.Add(index);
                    }
                    if (j < columns - 1)
                    {
                        index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i + 1, j + 1 })).Value;

                        if (index != -1 && !takenIndices.Contains(index))
                        {
                            partNums.Add(normalizedNumbers[index]);
                            takenIndices.Add(index);
                        }
                    }
                    if (j > 0)
                    {
                        index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i + 1, j - 1 })).Value;

                        if (index != -1 && !takenIndices.Contains(index))
                        {
                            partNums.Add(normalizedNumbers[index]);
                            takenIndices.Add(index);
                        }
                    }
                }
                if (i > 0)
                {
                    int index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i - 1, j })).Value;

                    if (index != -1 && !takenIndices.Contains(index))
                    {
                        partNums.Add(normalizedNumbers[index]);
                        takenIndices.Add(index);
                    }
                    if (j < columns - 1)
                    {
                        index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i - 1, j + 1 })).Value;

                        if (index != -1 && !takenIndices.Contains(index))
                        {
                            partNums.Add(normalizedNumbers[index]);
                            takenIndices.Add(index);
                        }
                    }
                    if (j > 0)
                    {
                        index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i - 1, j - 1 })).Value;

                        if (index != -1 && !takenIndices.Contains(index))
                        {
                            partNums.Add(normalizedNumbers[index]);
                            takenIndices.Add(index);
                        }
                    }
                }
                if (j < columns - 1)
                {
                    int index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i, j + 1 })).Value;

                    if (index != -1 && !takenIndices.Contains(index))
                    {
                        partNums.Add(normalizedNumbers[index]);
                        takenIndices.Add(index);
                    }
                }
                if (j > 0)
                {
                    int index = map.FirstOrDefault(x => x.Key.SequenceEqual(new int[2] { i, j - 1 })).Value;

                    if (index != -1 && !takenIndices.Contains(index))
                    {
                        partNums.Add(normalizedNumbers[index]);
                        takenIndices.Add(index);
                    }
                }

                if (partNums.Count == 2)
                    sum += partNums[0] * partNums[1];
            }
        }
        Console.WriteLine(sum);
    }

    /*public static void Main()
    {
        string txtPath = @"input3.txt";
        string[] lines = File.ReadAllLines(txtPath);
        char[,] map = new char[lines[0].Length, lines.Length];
        
        for (int i = 0; i < lines.Length; i++)
            for (int j = 0; j < lines[i].Length; j++)
                map[j, i] = lines[i].ElementAt(j);

        
    }*/
}
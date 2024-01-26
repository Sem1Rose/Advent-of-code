using System.Text.RegularExpressions;

class Day15
{
    static string txtPath = @"input15.txt";
    static string[] sequences;

    static void Start()
    {
        sequences = File.ReadAllText(txtPath).Split(',', StringSplitOptions.RemoveEmptyEntries).ToArray();

        Console.WriteLine(PartTwo());
        Console.Read();
    }

    static int PartOne()
    {
        int sum = 0;
        for (int i = 0; i < sequences.Length; i++)
            sum += Hash(sequences[i]);
        return sum;
    }

    static int PartTwo()
    {
        List<(string key, int val)>[] boxes = new List<(string key, int val)>[256];
        for (int i = 0; i < 256; i++)
            boxes[i] = new();

        for (int i = 0; i < sequences.Length; i++)
        {
            string label = Regex.Match(sequences[i], @"\w+").Value;
            int hashedLabel = Hash(label);
            char operation = sequences[i][label.Length];

            if (operation == '-')
            {
                if (boxes[hashedLabel].Any(x => x.key == label))
                    boxes[hashedLabel].RemoveAt(boxes[hashedLabel].IndexOf(boxes[hashedLabel].FirstOrDefault(x => x.key == label)));
            }
            else if (operation == '=')
            {
                int focalLength = int.Parse(sequences[i][(label.Length + 1)..]);
                if (!boxes[hashedLabel].Any(x => x.key == label))
                    boxes[hashedLabel].Add((label, focalLength));
                else
                    boxes[hashedLabel][boxes[hashedLabel].IndexOf(boxes[hashedLabel].FirstOrDefault(x => x.key == label))] = (label, focalLength);
            }
        }

        for (int j = 0; j < boxes.Length; j++)
        {
            for (int i = 0; i < boxes[j].Count; i++)
                Console.Write($"[{boxes[j][i].key} {boxes[j][i].val}] ");

            if (boxes[j].Count != 0)
                Console.WriteLine();
        }

        int focusingPowerSum = 0;
        for (int i = 0; i < 256; i++)
            for (int j = 0; j < boxes[i].Count; j++)
                focusingPowerSum += (i + 1) * (j + 1) * boxes[i][j].val;

        return focusingPowerSum;
    }

    static int Hash(string input)
    {
        int val = 0;
        for (int i = 0; i < input.Length; i++)
        {
            val += input[i];
            val = (val * 17) & 255;
        }
        return val;
    }
}
class Day7
{
    static readonly char[] orderedChars = new char[]{
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    };

    public static void Main()
    {
        string txtPath = @"input7.txt";
        string[] lines = File.ReadAllLines(txtPath);

        string[] handsStr = lines.Select(x => x.Split(" ")[0]).ToArray();
        string[] bidsStr = lines.Select(x => x.Split(" ")[1]).ToArray();

        int[] bids = bidsStr.Select(int.Parse).ToArray();
        int[] handsStrength = new int[handsStr.Length];
        int[] ranking = new int[handsStr.Length];

        long product = 0;
        for (int i = 0; i < handsStr.Length; i++)
        {
            string currHand = handsStr[i];
            int[] count = new int[5];
            int numJokers = 0;

            for (int j = 0; j < currHand.Length; j++)
            {
                char c = currHand[j];
                if (c == 'J')
                {
                    numJokers++;
                    continue;
                }

                if (currHand.IndexOf(c) != j)
                {
                    count[currHand.IndexOf(c)]++;
                    continue;
                }
                count[j] = 1;
            }

            List<int> countList = count.ToList();

            if (countList.Any(x => x == 5) || countList.Max() + numJokers == 5)
                handsStrength[i] = 6;
            else if (countList.Any(x => x == 4) || countList.Max() + numJokers == 4)
                handsStrength[i] = 5;
            else if ((countList.Any(x => x == 3) && countList.FindAll(x => x == 0).Count == 3) || (numJokers == 1 && countList.FindAll(x => x == 2).Count == 2))
                handsStrength[i] = 4;
            else if ((countList.Any(x => x == 3) && countList.FindAll(x => x == 0).Count == 2) || (numJokers + countList.Max() == 3))//(numJokers == 1 && countList.FindAll(x => x == 2).Count == 1) || (numJokers == 2 && countList.Max() == 1))
                handsStrength[i] = 3;
            else if (countList.FindAll(x => x == 2).Count == 2)
                handsStrength[i] = 2;
            else if ((countList.FindAll(x => x == 2).Count == 1 && countList.FindAll(x => x == 0).Count == 1) || (numJokers + countList.Max() == 2))
                handsStrength[i] = 1;
            else if (countList.FindAll(x => x == 1).Count == 5)
                handsStrength[i] = 0;
        }
        handsStrength.CopyTo(ranking, 0);

        //handsStrength.ToList().ForEach(x => Console.Write(x + " "));
        //Console.WriteLine();

        for (int a = 0; a < 1; a++)
        {
            for (int i = 0; i < ranking.Length; i++)
            {
                for (int j = 0; j < ranking.Length; j++)
                {
                    if (j == i)
                        continue;
                    if (ranking[i] == ranking[j])
                    {
                        for (int k = 0; k < handsStr[i].Length; k++)
                        {
                            char curr = handsStr[i][k];
                            char opp = handsStr[j][k];
                            if (curr == opp)
                                continue;

                            int currIndex = Array.IndexOf(orderedChars, curr);
                            int oppIndex = Array.IndexOf(orderedChars, opp);

                            if (currIndex < oppIndex)
                            {
                                ranking[j]--;
                                Reorder(j, ref ranking, handsStrength);
                                break;
                            }
                            else if (currIndex > oppIndex)
                            {
                                ranking[i]--;
                                Reorder(i, ref ranking, handsStrength);
                                break;
                            }
                        }
                    }
                }
            }
            HashSet<int> dups = new();
            foreach (var item in ranking)
            {
                if (!dups.Add(item))
                {
                    a--;
                    break;
                }
            }
        }

        int rankingMin = ranking.Min();
        int add = 0;
        if (rankingMin < 0)
            add = Math.Abs(rankingMin) + 1;
        if (rankingMin == 0)
            add = 1;
        if (rankingMin > 0)
            add = -rankingMin + 1;

        for (int i = 0; i < ranking.Length; i++)
        {
            ranking[i] += add;
            product += ranking[i] * bids[i];
        }

        int[] ass = new int[ranking.Length];
        ranking.CopyTo(ass, 0);

        Array.Sort(ranking, handsStr);
        Array.Sort(ass, bids);

        //for (int i = 0; i < handsStr.Length; i++)
        //    Console.WriteLine(handsStr[i] + "  " + bids[i] + " " + ranking[i]);

        Console.WriteLine(product);
    }

    static void Reorder(int indx, ref int[] ranking, int[] strengths)
    {
        for (int i = 0; i < ranking.Length; i++)
        {
            if (i == indx)
                continue;
            int val = ranking[i];
            if (ranking[indx] == val)
            {
                if (strengths[indx] == strengths[i])
                    continue;
                for (int j = 0; j < ranking.Length; j++)
                {
                    if (ranking[j] <= val && strengths[j] < strengths[indx])
                        ranking[j]--;
                }
                Reorder(i, ref ranking, strengths);
            }
        }
    }
}
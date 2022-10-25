/*Given a stack of N elements, interleave the first half of the stack with the 
second half reversed using only one other queue. This should be done in-place.*/
using System;
using System.Collections;
public class Program
{
    public static void Main(string[] args)
    {
        int[] input = {1,2,3,4};
        Stack stack = new Stack(input.Length);
        Queue queue = new Queue(input.Length - 1);
        foreach (int i in input)
        {
            stack.Push(i);
        }
        for (int i = input.Length - 1; i > 1; i--)
        {
            for (int n = 0; n<i ; n++)
            {
                queue.Enqueue(stack.Pop());
            }
            for (int n = 0; n<i;n++)
            {
                stack.Push(queue.Dequeue());
            }
        }

        for (int i = 0; i < input.Length; i++)
        {
            Console.WriteLine(stack.Pop());
        }
    }
}
use std::cmp::PartialOrd;

/// Recursively iterates and selects an element from the vector, comparing it
/// against the current largest value in the sorted list behind it. If the
/// value is larger, it moves on to the next element. If the value is smaller,
/// it then searches for the correct position to insert that value, shifting
/// all larger elements to the right by one to make room.
pub fn insertion_sort<T: Copy + PartialOrd>(data: &mut Vec<T>, n: usize)
{
    // Time Complexity => O(n) || O(n^2)
    // Space Complexity => O(n)

    if n > 0
    {
        insertion_sort::<T>(data, n - 1);

        let x: T = data[n];
        let mut j: usize = n - 1;

        while (j >= 0) && (data[j] > x)
        {
            data.swap(j + 1, j);
            j = j.saturating_sub(1);
        }

        data[j + 1] = x;
    }
}
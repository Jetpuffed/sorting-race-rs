use std::cmp::PartialOrd;

/// Iterates and selects an element from the vector, comparing it against
/// the current largest value in the sorted list behind it. If the value
/// is larger, it moves on to the next element. If the value is smaller,
/// it then searches for the correct position to insert that value, shifting
/// all larger elements to the right by one to make room.
pub fn insertion_sort<T: Copy + PartialOrd>(data: &mut Vec<T>)
{
    // Time Complexity => O(n) || O(n^2)
    // Space Complexity => O(1)

    let mut i: usize = 1;

    while i < data.len()
    {
        let x: T = data[i];
        let mut j: usize = i - 1;

        while (j >= 0) && (data[j] > x)
        {
            data.swap(j + 1, j);
            j = j.saturating_sub(1);
        }

        data[j + 1] = x;
        i += 1;
    }
}

pub fn selection_sort<T: PartialOrd>(data: &mut Vec<T>)
{
    let size: usize = data.len();

    for i in 0 .. (size - 1)
    {
        let mut min: usize = i;

        for j in (i + 1) .. size
        {
            if data[j] < data[min]
            {
                min = j;
            }
        }

        if min != i
        {
            data.swap(i, min);
        }
    }
}

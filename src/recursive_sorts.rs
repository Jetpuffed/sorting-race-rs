use std::cmp::PartialOrd;

pub fn insertion_sort<T: Copy + PartialOrd>(data: &mut Vec<T>, n: usize)
{
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
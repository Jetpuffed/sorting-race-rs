use std::cmp::PartialOrd;

pub fn insertion_sort<T: Copy + PartialOrd>(data: &mut Vec<T>)
{
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

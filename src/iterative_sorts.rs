use std::cmp::PartialOrd;

pub fn insertion_sort<T: PartialOrd>(data: &mut Vec<T>)
{
    let mut i: usize = 1;

    while i < data.len()
    {
        let mut j: usize = i;

        while (j > 0) && (data[j - 1] > data[j])
        {
            data.swap(j, j - 1);
            j = j.saturating_sub(1);
        }

        i += 1;
    }
}

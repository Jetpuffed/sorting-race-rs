use std::cmp::PartialOrd;

// =================
//  Insertion Sorts
// =================

pub fn cube_sort() {}

pub fn insertion_sort<T: PartialOrd>(mut input: Vec<T>)
{
    let mut i = 1;

    while i < input.len()
    {
        println!("{}", i);
        let x = unsafe { std::ptr::read(&input[i]) };
        let mut j = i - 1;

        while input[j] > x
        {
            input.swap(j, j + 1);
            j = j.saturating_sub(1);
        }

        input[j + 1] = x;
        i += 1;
    }
}

pub fn library_sort() {}

pub fn shell_sort() {}

pub fn tree_sort() {}

// =================
//  Selection Sorts
// =================

pub fn cycle_sort() {}

pub fn heap_sort<T: PartialOrd>(mut input: Vec<T>)
{
    let mut start = (((input.len() - 1) - 1) / 2) as i32;
    let mut end = input.len() - 1;

    while start >= 0
    {
        let mut root = start as usize;

        while (2 * root) + 1 <= end
        {
            let child = (2 * root) + 1;
            let mut swap = root;

            if input[swap] < input[child]
            {
                swap = child;
            }

            if (child + 1 <= end) && (input[swap] < input[child + 1])
            {
                swap = child + 1;
            }

            if swap == root
            {
                break;
            }
            else
            {
                input.swap(root, swap);
                root = swap;
            }
        }

        start -= 1;
    }

    while end > 0
    {
        input.swap(end, 0);
        end -= 1;

        let mut root = 0;

        while (2 * root) + 1 <= end
        {
            let child = (2 * root) + 1;
            let mut swap = root;

            if input[swap] < input[child]
            {
                swap = child;
            }

            if (child + 1 <= end) && (input[swap] < input[child + 1])
            {
                swap = child + 1;
            }

            if swap == root
            {
                break;
            }
            else
            {
                input.swap(root, swap);
                root = swap;
            }
        }
    }
}

pub fn selection_sort<T: PartialOrd>(mut input: Vec<T>)
{
    let length = input.len();

    for i in 0 .. length - 1
    {
        let mut min = i;

        for j in i + 1 .. length
        {
            if input[j] < input[min]
            {
                min = j;
            }
        }

        if i != min
        {
            input.swap(i, min);
        }
    }
}

pub fn smooth_sort() {}

pub fn strand_sort() {}

pub fn tournament_sort() {}

// ===============
//  Merging Sorts
// ===============

pub fn merge_sort<T: PartialOrd>(input: Vec<T>)
{
    let mut output = Vec::with_capacity(input.len());
    let mut vec_stk = vec![input];

    while let Some(tmp) = vec_stk.pop()
    {
        if tmp.len() <= 1
        {
            output.push(tmp);

            if output.len() == 2
            {
                let (mut tmp_l, mut tmp_r) = (output.remove(0), output.remove(0));
                let mut tmp_output = Vec::with_capacity(tmp_l.len() + tmp_r.len());

                while !tmp_l.is_empty() && !tmp_r.is_empty()
                {
                    if tmp_l.first() <= tmp_r.first()
                    {
                        tmp_output.push(tmp_l.remove(0));
                    }
                    else
                    {
                        tmp_output.push(tmp_r.remove(0));
                    }
                }

                while !tmp_l.is_empty()
                {
                    tmp_output.push(tmp_l.remove(0));
                }

                while !tmp_r.is_empty()
                {
                    tmp_output.push(tmp_r.remove(0));
                }

                output.push(tmp_output);
            }

            continue;
        }

        let (mut tmp_l, mut tmp_r) = (Vec::new(), Vec::new());

        for (i, val) in tmp.iter().enumerate()
        {
            if i < tmp.len() / 2
            {
                tmp_l.push(unsafe { std::ptr::read(val) });
            }
            else
            {
                tmp_r.push(unsafe { std::ptr::read(val) });
            }
        }

        (vec_stk.push(tmp_l), vec_stk.push(tmp_r));
    }
}

pub fn merge_sort_in_place() {}

// ==================
//  Exchanging Sorts
// ==================

pub fn bubble_sort() {}

pub fn cocktail_shaker_sort() {}

pub fn comb_sort() {}

pub fn exchange_sort() {}

pub fn gnome_sort() {}

pub fn odd_even_sort() {}

// =============
//  Mixed Sorts
// =============

pub fn block_sort() {}

pub fn intro_sort() {}

pub fn patience_sort() {}

pub fn tim_sort() {}

// =============
//  Other Sorts
// =============

pub fn quick_sort() {}

// ======================
//  Non-comparison Sorts
// ======================

pub fn bucket_sort() {}

pub fn burst_sort() {}

pub fn counting_sort() {}

pub fn flash_sort() {}

pub fn lsd_radix_sort() {}

pub fn msd_radix_sort() {}

pub fn pigeonhole_sort() {}

pub fn postman_sort() {}

pub fn spread_sort() {}

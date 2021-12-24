use std::mem::MaybeUninit;

pub fn iter_consume_exact<T, const N: usize>(
    mut iter: impl Iterator<Item = T>,
) -> Result<[T; N], &'static str> {
    let mut data = MaybeUninit::<[T; N]>::uninit();
    let start: *mut T = unsafe { std::mem::transmute(&mut data) };

    for i in 0..N {
        let item = iter.next().ok_or("Insufficient items in iterator")?;
        unsafe { start.offset(i as isize).write(item) }
    }

    if iter.next().is_some() {
        return Err("Too many items in iterator");
    }

    Ok(unsafe { data.assume_init() })
}

pub fn parse_prefix_and_split<'a, const N: usize>(
    s: &'a str,
    prefix: &str,
    split_pat: &str,
) -> Result<[&'a str; N], &'static str> {
    if !s.starts_with(prefix) {
        return Err("Invalid prefix");
    }

    iter_consume_exact(s[prefix.len()..].split(split_pat))
}

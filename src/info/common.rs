pub fn ipv4_to_int(s: &str) -> u32
{
    let mut octets = s.split('.').filter_map(|octet| {
        let trimmed = octet.trim();
        if trimmed.is_empty()
        {
            None
        }
        else
        {
            Some(trimmed.parse::<u32>().unwrap_or_default())
        }
    });

    let a = octets.next().unwrap_or_default();
    let b = octets.next().unwrap_or_default() << 8;
    let c = octets.next().unwrap_or_default() << 16;
    let d = octets.next().unwrap_or_default() << 24;

    u32::to_be(a + b + c + d)
}

pub fn int_to_ipv4(i: u32) -> String
{
    use std::io::Write;
    let i = i.to_le();
    let mut buf = [0u8; 15];
    {
        let mut cursor = std::io::Cursor::new(&mut buf[..]);
        write!(
            cursor,
            "{}.{}.{}.{}",
            i & 0xFF,
            (i >> 8) & 0xFF,
            (i >> 16) & 0xFF,
            (i >> 24) & 0xFF
        )
        .unwrap();
    }
    String::from_utf8_lossy(&buf[..]).to_string()
}

/// An iterator that produces only the unique elements from the iterator it was
/// called on
struct Unique<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    iter: I,
    seen: Vec<String>,
}

impl<'a, I> Iterator for Unique<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item>
    {
        for item in &mut self.iter
        {
            let item_str = item.to_owned();
            if !self.seen.contains(&item_str)
            {
                self.seen.push(item_str);
                return Some(item);
            }
        }
        None
    }
}

/// An extension trait that adds a `unique` method to the `Iterator` trait
trait UniqueIterator<'a>: Iterator<Item = &'a str>
{
    fn unique(self) -> Unique<'a, Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            seen: Vec::new(),
        }
    }
}

impl<'a, I> UniqueIterator<'a> for I where I: Iterator<Item = &'a str> {}

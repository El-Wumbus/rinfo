mod info;
mod printing;

fn main()
{
    let info = info::Info::read().unwrap();
    dbg!(info);
}

mod info;
mod printing;

fn main()
{
    printing::print_with_logo(
        printing::OsArt::ArchLinux,
        "Wow, This works.\nIt actually works and this is great.\nl\nl\nl",
    );
    printing::print_with_logo(
        printing::OsArt::AlpineLinux,
        "Wow, This works.\nIt actually works and this is great.\nl\nl\nl",
    );
}

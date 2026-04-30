fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/resources/icon.ico");
    res.set("FileDescription", "Tacky Borders");
    res.set("ProductName", "Tacky Borders");
    res.compile().unwrap();
}

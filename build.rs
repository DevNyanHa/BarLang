fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().expect("아이콘 컴파일 실패");
}
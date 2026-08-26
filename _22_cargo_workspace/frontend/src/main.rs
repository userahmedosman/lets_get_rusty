use shared::ProjectInfo;

fn main() {
    let info = ProjectInfo::show(
        "Frontend project".to_owned(),
        1.0
    );
    println!("{info:?}");
}

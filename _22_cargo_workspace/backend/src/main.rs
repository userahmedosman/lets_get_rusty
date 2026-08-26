use shared::ProjectInfo;
fn main() {
     let info = ProjectInfo::show(
        "Backend project".to_owned(),
        1.0
    );
    println!("{info:?}");
}

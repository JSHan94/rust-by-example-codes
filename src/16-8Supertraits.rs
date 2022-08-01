// Rust doesn't support `inheritance`
// But you can define a trait as being superset of another trait

trait Person {
    fn name(&self) -> &'static str;
}
trait Student: Person{
    fn university(&self) -> &'static str;
}
trait Programmer{
    fn fav_language(&self) -> &'static str;
}
trait CompSciStudent : Programmer + Student{
    fn git_username(&self) -> &'static str;
}
struct Me{
    name : &'static str,
    university : &'static str,
    fav_language : &'static str,
    git_username : &'static str
}
impl CompSciStudent for Me{
    fn git_username(&self) -> &'static str{
        self.git_username
    }
}
impl Programmer for Me{
    fn fav_language(&self) -> &'static str{
        self.fav_language
    }
}
impl Student for Me{
    fn university(&self) -> &'static str{
        self.university
    }
}
impl Person for Me{
    fn name(&self) -> &'static str{
        self.name
    }
}
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String{
    format!("name : {}, University : {}, fav_language : {}, Git username : {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main(){
    let student = Me{
        name : "Harvey",
        university : "Postech",
        fav_language : "Rust",
        git_username : "JSHan94"
    };
    println!("{}", comp_sci_student_greeting(&student)); 
}
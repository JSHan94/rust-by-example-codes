// configuration conditional checks
// #[cfg(...)] : attribute position
// cfg!(... ) : boolean expression

// only compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux(){
    println!("You are not running linux!");
}

#[cfg(some_condition)]
fn conditional_function(){
    println!("condition met!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux"){
        println!("Yes, It's definitely linux!");
    }else {
        println!("No, It's not linux");
    }

    conditional_function(); // rustc --cfg some_condition 13-3cfg.rs && ./13-3cfg
}

fn main() {
    let vec = vec![1, 2];
    for element in &vec {
        println!("Element: {:?}", element);
    }
    //Alternatively, clone the vec to preserve original and iterate
    // let vec_clone = vec.clone(); 
    // for element in vec_clone {
    //     println!("Element: {:?}", element);
    // }
} 
// Clone
//fn main() {
  //  let x = 5;
    //let y = x; // This creates a copy for primitive types
    //println!("x is: {}, y is: {}", x, y);

    //let s1 = String::from("Hello");
    //let s2 = s1.clone(); // This moves ownership, s1 is no longer valid
    
    // println!("s1 is: {}, s2 is: {}", s1, s2); // This would cause a compile error
//}

// fn main() {
//     let s = String::from("hello");
//     let s = takes_ownership(s);
//     // s is no longer valid here
//     println!("{}",s)
// }

// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     some_string
// } // some_string goes out of scope and is dropped



// // Borrowing
// fn borrowing_doesnot_move_ownership() {
//     let word = "UTRGV".to_string();
//     let borrow_word = &word;
//     println!("{} == {}", word, borrow_word);
// }


// fn borrowing(){
//     let first = "RGV".to_string();
//     let borrow_first = &first;
//     println!("{} == {}", first, borrow_first);
// }

// fn main(){
//     let word = "UTRGV".to_string();
//     let borrow_word = &word;

//     let borrow_word1 = &word;

//     print_string(&word);
//     print_string(&borrow_word1);

// }

// fn print_string(w:& String){
//     println!("{}",w);
// }


// Inmuttable Borrowing
// fn if_no_update_borrow_to_read_best_option() {
//     fn my_print(word: &String) {
//         println!("{}", word);
//     }
//     let word = "UTRGV".to_string();
//     my_print(&word);
// }

//Mutable Borrowing
fn borrow_to_mut_watchout() {
    let mut word = "UT".to_string(); 
    fn update(word: &mut String) {
        word.push_str("RGV");
    }
    update(&mut word);
    println!("{word}")
}

fn main(){
    borrow_to_mut_watchout();
}


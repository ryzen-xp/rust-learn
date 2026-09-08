// Topic: Rectangle_into_Squares
// Run with:  cargo run --bin Rectangle_into_Squares

fn main() {
    println!("Hello World : cargo run --bin Rectangle_into_Squares");
}

fn sq_in_rect(mut lng: i32, mut wdth: i32) -> Option<Vec<i32>> {

    if lng == wdth  {
        return  None;
    }

    let  mut vec = Vec::new();
    

    while lng != wdth {

        if lng > wdth {
            vec.push(wdth);

            lng -=wdth;
        }

        else {
            vec.push(lng);
            wdth-=lng;
        }
        
    }

    vec.push(lng);

    Some(vec)
}


fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {

    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
  
}

//macro_rules! vec {
//    ( $x:expr ) => {{
//        let mut temp_vec = Vec::new();
//        temp_vec.push($x);
//        temp_vec
//    }};
//}

//macro_rules! vec {
//    ( $( $x:expr ),* ) => {
//        {
//            let mut temp_vec = Vec::new();
//            $(
//                temp_vec.push($x);
//            )*
//            temp_vec
//        }
//    };
//}

macro_rules! vec {
    ( $x:ty ) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let x = vec![0];
    let y = vec![0, 1, 2];
    let z = vec![i32];
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

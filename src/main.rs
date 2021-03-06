macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident : $type:ty),*) -> $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            let mut _memoize = (true,$($pname),*);
            while _memoize.0 {
                println!("Test complete...");
                fn recur (repeat:bool,$($pname:$type),*) -> (bool,$($type),*) {(true,$($pname),*)}
                _memoize = {$fbody};
            }
            fn arb_fn ($($pname : $type),*) -> $rtrn_type {$fbody}
            println!("{:?}", arb_fn(_memoize.0));
    });
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

// CURRENT ERROR : UNABLE TO OVERLOAD FUNCTION (ALTHOUGH THERE ARE WAYS TO GET AROUND IT), CAN'T HAVE AMBIGUOUS RETURN STATEMENT

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 10 {
           println!("Okay");
           recur(_num1 + 1, _num2 + 1)
       } else {
            sum
       }
    }   
}

fn main() {
   println!("Tail recursive macro almost done!!!");
   //println!("{:?}", add_together(0, 0));
}

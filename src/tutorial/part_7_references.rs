pub fn main() {
    /*
        References Rules:
         - Only one &mut ref in a scope
         - More than one &ref are allowed
         - &mut ref and &ref can not coexist
         - Scope of a ref is till it is being
           used in a stmt or expr
         - Data should not change when an immutable
           ref is in scope
    */
    let mut var = 23;
    let mut_ref1 = &mut var; // scope of mut_ref1 starts
                             /*

                             > second &mut ref is not allowed while another
                               &mut ref is in scope

                             let mut_ref2 = &mut var;

                             > third immutable ref_3 is not allowed here while another
                               &mut ref is in scope

                             let ref_3 = &var;

                             println!(
                                 "mut_ref1 : {:?} mut_ref2 : {:?} immutable ref_3 : {:?}",
                                 mut_ref1, mut_ref2, ref_3
                             );

                             */
    println!("mut_ref1 : {:?}", mut_ref1); // scope of mut_ref1 ends as last use here

    let mut_ref4 = &mut var; // allowed since mut_ref1 scope is over

    // var=5; data cannot change when immutable ref mut_ref4 is in scope

    println!("mut_ref4 : {:?}", mut_ref4);
}

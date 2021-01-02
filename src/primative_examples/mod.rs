mod arrays_ex;
mod literal_operators;
mod primativeexamples;
mod tuple_examples;

pub fn example_prim() {
    
    //literals and operators examples
    literal_operators::literal_ex01();
    //other primatives
    primativeexamples::hello("jefe");
    primativeexamples::add(2, 3);
    
    //Tuples
    tuple_examples::tuple_allexamples();
    
    //arrays
    arrays_ex::array_ex_all();
}

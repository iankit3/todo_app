/*  - Things i don't know or learn't after completing this app
    -- Complex data types (struct , class );
    -- imports 
    -- scanf    
*/


fn main() {
    /*
        @args
        name:&str , is_done:bool , due_date:&str 
    */
    let todo : (&str , bool , &str);
        todo = ("Create a todo console app in rust",false,"15th August");
    println!("{:?}",todo);
}



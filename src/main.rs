/*  - Things i don't know or learn't after completing this app
    -- Complex data types (struct , class );
    -- imports 
    -- scanf    
*/

// Learn't about Lifetime  : missing lifetime specifier - expected identifier
#[derive(Debug)]
struct Task<'a>{
    task_name : &'a str,
    is_done : bool,
    due_date : &'a str 
}

fn main() {    
    let todo : Task;
    todo = Task { 
        task_name : "Create a todo console app in rust" ,
        is_done : false ,
        due_date : "15th August" 
    };
    println!("{:?}",todo);
}



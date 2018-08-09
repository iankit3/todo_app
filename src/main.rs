/*  - Things i don't know or learn't after completing this app
    -- Complex data types (struct , class );
    -- imports 
    -- scanf    
*/

// Learn't about Lifetime  : missing lifetime specifier - expected identifier
#[derive(Debug)]
pub struct Task<'a>{
    task_name : &'a str,
    is_done : bool,
    due_date : &'a str 
}


fn main() {    
    let tasks_list : Vec<Task> = Vec::new();
    let todo : Task;
    todo = Task { 
        task_name : "Create a todo console app in rust" ,
        is_done : false ,
        due_date : "15th August" 
    };

    add_task(tasks_list,todo);
    /*
        print_tasks(tasks_list);
   |                 ^^^^^^^^^^ value used here after move
     */
    //print_tasks(tasks_list);
}

fn add_task<'a>(mut tasks_list : Vec<Task<'a>> , todo : Task<'a>) /*-> Vec<Task<'a>>*/{
    &tasks_list.push(todo);
    print_tasks(tasks_list);
    //tasks_list
}

fn print_tasks(tasks_list : Vec<Task>){
    for task in tasks_list  {
        println!("{:?}", task);
    }
}





/*  - Things i don't know or learn't after completing this app
    -- Complex data types (struct , class );
    -- imports 
    -- scanf    
*/

pub trait Print {
    fn pretty_print(&self) -> ();
}

// Learn't about Lifetime  : missing lifetime specifier - expected identifier
#[derive(Debug)]
pub struct Task<'a>{
    task_name : &'a str,
    is_done : bool,
    due_date : &'a str 
}

impl<'a> Print for Task<'a>{
    fn pretty_print(&self) -> (){
        println!("Task Name : {} ,with status {} and a due date of {}",&self.task_name , &self.is_done , &self.due_date)
    }
}


fn main() {    
    let mut tasks_list : Vec<Task> = Vec::new();
    let todo : Task;
    todo = Task { 
        task_name : "Create a todo console app in rust" ,
        is_done : false ,
        due_date : "15th August" 
    };

    add_task(&mut tasks_list,todo);
    add_task(&mut tasks_list, Task { 
                    task_name : "Work out to make it look fine." ,
                    is_done : false , 
                    due_date : "17th August" 
                });
    print_tasks(tasks_list);
}

fn add_task<'a>(tasks_list : &mut Vec<Task<'a>>, todo : Task<'a>){
    tasks_list.push(todo);
}

fn print_tasks(tasks_list : Vec<Task>){
    for task in tasks_list  {
        task.pretty_print();
    }
}





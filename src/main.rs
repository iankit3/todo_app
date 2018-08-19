/*  - Things i don't know or learn't after completing this app
    -- Complex data types (struct , class );
    -- imports 
    -- scanf    
*/
use std::cmp::Ordering;

pub trait Print {
    fn pretty_print(&self) -> ();
}


// Learn't about Lifetime  : missing lifetime specifier - expected identifier
#[derive(Debug,Copy,Clone,Eq)]
pub struct Task<'a>{
    task_name : &'a str,
    is_done : bool,
    due_date : &'a str 
}

impl<'a> PartialEq<Task<'a>> for Task<'a>{
    fn eq(&self, other: &Task) -> bool{
        self.task_name == other.task_name
    }

    fn ne(&self, other: &Task) -> bool {  
        self.task_name != other.task_name
    }
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
    remove_task(&mut tasks_list, todo);
    print_tasks(tasks_list);
}

fn add_task<'a>(tasks_list : &mut Vec<Task<'a>>, todo : Task<'a>){
    tasks_list.push(todo);
}

fn sort_task(task_list : &mut Vec<Task>){
    //implement sort
    task_list.sort();
}

fn filter_task(task_list : Vec<Task>){
    //implement filter
    unimplemented!();
    task_list = task_list
                .iter()
                .filter(|x| x.is_done == true)
                .collect();
}

fn delete_task(task_list : Vec<Task>){
   unimplemented!();
}

fn mark_all_tasks_done(tasks_list : &mut Vec<Task>){
    tasks_list = tasks_list.iter().map(|task| task.is_done = true).collect();
}

fn remove_task<'a>(tasks_list : &mut Vec<Task<'a>> , todo : Task<'a>){
   let index = tasks_list.iter().position(|&td| td.task_name == todo.task_name).unwrap();
    println!("A Task at {} has been removed from the task_list",index);
    tasks_list.remove(index);
}

fn print_tasks(tasks_list : Vec<Task>){
    for task in tasks_list  {
        task.pretty_print();
    }
}





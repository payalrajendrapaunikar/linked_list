

use simple_linked_list::LinkedList;

#[derive(Debug,PartialEq)]
pub struct User{
    pub name : String,
    pub roll_no : i32,
}

pub fn main(){

    let mut  linked_list : LinkedList<i32> = LinkedList::new();

    linked_list.push(1);
    linked_list.push(2);

    linked_list.pop();    

    linked_list.print_list();

    let length = linked_list.len();
    println!("the lenght of linked list is {}",length);


    let user1 = User{
        name : "Payal Paunikar".to_string(),
        roll_no : 1,
    };

    let user2 = User{
        name : "Prachi Paunikar".to_string(),
        roll_no : 2,
    };

    let mut user_list : LinkedList<User> = LinkedList::new();

    user_list.push(user1); 
    user_list.push(user2);
  
    user_list.print_list();
    let length = user_list.len();
    println!("the lenght of user list is {}",length);


}
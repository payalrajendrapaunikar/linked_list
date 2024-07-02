
//! Structs



/// Each `Node` contains a piece of data and an optional reference
/// to the next node in the list. The reference to the next node
/// is wrapped in an `Option` and a `Box`, which allows for nodes
/// to be heap-allocated and linked together dynamically.
#[derive(Debug,PartialEq,Clone)]
pub struct Node<T>{
   pub data : T,
   pub next : Option<Box<Node<T>>>
}

/// The `LinkedList` structure is represent the singly linked list.
/// The struct provide methods to create and manipulate the linked list.
#[derive(Debug,PartialEq,Clone)]
pub struct LinkedList<T>{
   pub head : Option<Box<Node<T>>>
}

use std::fmt::Debug; 

impl<T:Debug> LinkedList<T> {
    
    /// Created a new empty linked list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linked_list::LinkedList;
    /// 
    /// let linked_list : LinkedList<i32>  = LinkedList::new();
    /// ```
    pub fn new()-> Self{
        LinkedList { head : None}
    }

    /// Check if the linked list is empty or not.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linked_list::LinkedList;
    /// 
    /// let linked_list : LinkedList<i32> = LinkedList::new();
    /// assert!(linked_list.is_empty());
    /// ```
    pub fn is_empty(&self)-> bool{
        self.head.is_none()
    }


    /// The 'push' method adds a new node to the top of the linked list.
    /// # Example 
    /// 
    /// ```
    /// 
    /// use linked_list::LinkedList; 
    /// 
    /// #[derive(Debug,PartialEq)]
    /// pub struct User{
    ///  name : String,
    ///  roll_no : u32,
    /// }
    /// 
    ///  //create some users
    /// let user1 = User{
    ///    name : String::from("Payal Paunikar"),
    ///    roll_no : 1,
    /// };
    /// 
    /// let user2 = User{
    ///    name : String::from("Prachi Paunikar"),
    ///    roll_no : 2,
    /// };
    /// 
    ///  //create a empty linked list
    /// let mut user_linked_list : LinkedList<User>= LinkedList::new();
    /// 
    /// //push user into the linked list
    /// user_linked_list.push(user1);
    /// user_linked_list.push(user2);
    /// ```
    pub fn push(&mut self, data : T){

       let new_node = Box::new(
         Node{
            data,
            next : self.head.take(),
         }
       );

       self.head = Some(new_node);
    }


     /// The 'pop' method remove head node from the linked list
     /// 
     /// # Example
     /// 
     /// ```
    /// 
    /// use linked_list::LinkedList; 
    /// 
    /// #[derive(Debug,PartialEq)]
    /// pub struct User{
    ///  name : String,
    ///  roll_no : u32,
    /// }
    /// 
    ///  //create some users
    /// let user1 = User{
    ///    name : String::from("Payal Paunikar"),
    ///    roll_no : 1,
    /// };
    /// 
    /// let user2 = User{
    ///    name : String::from("Prachi Paunikar"),
    ///    roll_no : 2,
    /// };
    /// 
    ///  //create a empty linked list
    /// let mut user_linked_list =LinkedList::new();
    /// 
    /// //push user into the linked list
    /// user_linked_list.push(user1);
    /// user_linked_list.push(user2);
    /// 
    /// //pop the user from the linked list
    /// user_linked_list.pop();
    /// ``` 
     pub fn pop(&mut self){
        
        self.head.take().map(
            |node|
            self.head = node.next
        );
     }


     /// The 'print_list' method print the linked list
     /// 
      /// # Example 1 
     /// 
     /// ``` 
    /// use linked_list::LinkedList; 
    /// 
    /// #[derive(Debug,PartialEq)]
    /// pub struct User{
    ///  name : String,
    ///  roll_no : u32,
    /// }
    /// 
    ///  //create some users
    /// let user1 = User{
    ///    name : String::from("Payal Paunikar"),
    ///    roll_no : 1,
    /// };
    /// 
    /// let user2 = User{
    ///    name : String::from("Prachi Paunikar"),
    ///    roll_no : 2,
    /// };
    /// 
    ///  //create a empty linked list
    /// let mut user_linked_list = LinkedList::new();
    /// 
    /// //push user into the linked list
    /// user_linked_list.push(user1);
    /// user_linked_list.push(user2);
    /// 
    /// //print the linked list
    /// user_linked_list.print_list();
    /// 
    /// ``` 
    /// 
    /// # Example 2
    /// 
    /// ```
    /// use linked_list::LinkedList;
    /// 
    /// let mut linked_list : LinkedList<i32> = LinkedList::new();
    /// linked_list.push(1);
    /// linked_list.push(2);
    /// 
    /// linked_list.print_list();
    /// 
    /// ```
     pub fn print_list(&self){
         
        let mut current_node = &self.head;
      

        while let Some(node) = current_node {
            print!("{:?}->",node.data);
            current_node = &node.next;
        }
     
         println!("None");
       
     }


     /// The 'len' method count the number of element store in the list
     /// 
     /// # Example
     /// ```
     /// use linked_list::LinkedList;
     /// let mut number_list : LinkedList<u32> = LinkedList::new();
     /// number_list.push(1);
     /// number_list.push(2);
     /// 
     /// let number_list_count = number_list.len();
     /// assert_eq!(2,number_list_count);
     /// ```
     pub fn len(&self)->usize{
         let mut count = 0;
         let mut current_node = &self.head;
         while let Some(node) = current_node {
             count += 1;
             current_node = &node.next;
         }
         count

     }


    
     
}




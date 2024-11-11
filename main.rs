use std::{arch::x86_64::_MM_GET_EXCEPTION_MASK, string};
#[derive(Clone,Debug)]
struct Member {
    name:String,
    password:String,
    email:String,
    tc_no:String,
    age:u32,
    balance:i32
   

}


fn register (name:String,password:String,email:String,tc_no:String,age:u32,balance:i32,database :&mut Vec<Member>) {

    if  !email.contains("@") || age<18 {
        println!("Hatalı giriş yaptınız!!")
    
    }
    else {
            let member = Member {
                name:name,
                password:password,
                tc_no:tc_no,
                email:email,
                age:age,
                balance:balance


            };
            database.push(member)
    }


}

fn login (email:String, password:String,database :&mut Vec<Member>) {

    for  data in database {
        if email== data.email && password== data.password {
            println!("Başarıyla giriş yapıldı");
            println!("Hesap ismi : {}",data.name);
            println!("Hesaptaki mail adresini : {}",data.email);
            println!("Hesap Bakiyeniz : {}",data.balance);

            return;
            
        }
        else {
            println!("Hatalı giriş yaptınız!!!");
           


        
        }
        
    }
}


fn main () {
    let mut database: Vec<Member>=Vec::new();
    
    
     register("satı samet körcan".to_string(), "samet123".to_string(), "sametkorcan@icloud.com".to_string()
        ,"12345678900".to_string(),19, 1200,&mut database);

    login("sametkorcan@icloud.com".to_string(), "samet123".to_string(), &mut database);

}


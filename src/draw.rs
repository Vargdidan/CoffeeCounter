pub fn draw (content: String) {
    coffee_registered(content);
    coffee_menu();
}

fn coffee_registered(mut content: String) {
    let coffee_string = 
"        (  )   (   )  )
         ) (   )  (  (
         ( )  (    ) )
         _____________
        <_____________> ___
        |     *    |/ _ \\
        |   coffee(s)   | | |
        |  registered   |_| |
      __|             |\\___/
     /  \\____________/  \\
     \\__________________/";

     if content.is_empty() {
         content = "0".to_string();
     }

    let mut counter = 4 - content.len();
    while counter != 0 {
        content.push_str(" ");
        counter -= 1;
    }

    println!("{}", coffee_string.replace("*", content.as_str()));
}

fn coffee_menu() {
    let coffee_menu_string =
"    ________________________
    |    (R)    |    (Q)    |
    |  Register |   Quit    |
    -------------------------";
    println!("{}", coffee_menu_string);
}
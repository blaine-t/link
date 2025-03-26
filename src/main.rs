use link::*;

fn main() {
    let env = create_env();
    let db = create_db(&env);
    let rtxn = create_rtxn(&env);
    
    let key = "traudtDev";

    let mut i: u128 = 0;
    loop {
        if let Err(e) = add_kvp(db, &env, &i.to_string(), &i.to_string()) {
            eprintln!("Error adding key-value pair: {}", e);
            break;
        }
        i += 1;
        println!("{:?}", get_kvp(db, &rtxn, &i.to_string()));
    }
    println!("{:?}", get_kvp(db, &rtxn, key));
}
